package main

import (
	"C"
	"encoding/json"
	http_base "net/http"
	"os"
	"runtime"
	"time"
	"sync/atomic"
	"github.com/couchbase/query/accounting"
	acct_resolver "github.com/couchbase/query/accounting/resolver"
	config_resolver "github.com/couchbase/query/clustering/resolver"
	"github.com/couchbase/query/datastore"
	"github.com/couchbase/query/datastore/resolver"
	"github.com/couchbase/query/datastore/system"
	"github.com/couchbase/query/errors"
	"github.com/couchbase/query/execution"
	"github.com/couchbase/query/functions/constructor"
	"github.com/couchbase/query/logging"
	log_resolver "github.com/couchbase/query/logging/resolver"
	"github.com/couchbase/query/prepareds"
	"github.com/couchbase/query/server"
	"github.com/couchbase/query/server/http"
	"github.com/couchbase/query/timestamp"
	"github.com/couchbase/query/value"
)

type MockServer struct {
	server    *server.Server
	acctstore accounting.AccountingStore
	dstore    datastore.Datastore
}

type MockQuery struct {
	server.BaseRequest
	response    *MockResponse
	resultCount int
}

func (this *MockQuery) Execute(srvr *server.Server, signature value.Value) {
	select {
	case <-this.Results():
		this.stopAndAlert(server.COMPLETED)
	case <-this.StopExecute():
		this.stopAndAlert(server.STOPPED)

		// wait for operator before continuing
		<-this.Results()
	}
	close(this.response.done)
}

func (this *MockQuery) OriginalHttpRequest() *http_base.Request {
	return nil
}

func (this *MockQuery) Output() execution.Output {
	return this
}

func (this *MockQuery) Failed(srvr *server.Server) {
	this.stopAndAlert(server.FATAL)
}

func (this *MockQuery) Fail(err errors.Error) {
	defer this.Stop(server.FATAL)
	this.response.err = err
	close(this.response.done)
}

func (this *MockQuery) Expire(state server.State, timeout time.Duration) {
	defer this.stopAndAlert(state)

	this.response.err = errors.NewError(nil, "Query timed out")
	close(this.response.done)
}

func (this *MockQuery) stopAndAlert(state server.State) {
	this.Stop(state)
	this.Alert()
}

func (this *MockQuery) SetUp() {
}

func (this *MockQuery) Result(item value.AnnotatedValue) bool {
	bytes, err := json.Marshal(item)
	if err != nil {
		this.SetState(server.FATAL)
		panic(err.Error())
	}

	this.resultCount++

	var resultLine map[string]interface{}
	json.Unmarshal(bytes, &resultLine)

	this.response.results = append(this.response.results, resultLine)
	return true
}

type MockResponse struct {
	err      errors.Error
	results  []interface{}
	warnings []errors.Error
	done     chan bool
}

var GlobalServer atomic.Value

//export StartServer
func StartServer() {

	mockServer := &MockServer{}

	ds, err := resolver.NewDatastore("dir:json")
	if err != nil {
		println(err.Error())
		logging.Errorp(err.Error())
		os.Exit(1)
	}
	datastore.SetDatastore(ds)

	sys, err := system.NewDatastore(ds)
	if err != nil {
		logging.Errorp(err.Error())
		os.Exit(1)
	}

	configstore, err := config_resolver.NewConfigstore("stub:")
	if err != nil {
		logging.Errorp("Could not connect to configstore",
			logging.Pair{"error", err},
		)
	}

	acctstore, err := acct_resolver.NewAcctstore("stub:")
	if err != nil {
		logging.Errorp("Could not connect to acctstore",
			logging.Pair{"error", err},
		)
	}

	// Start the completed requests log - keep it small and busy
	server.RequestsInit(0, 8)

	// Start the prepared statement cache
	prepareds.PreparedsInit(1024)

	// need to do it before NewServer() or server scope's changes to
	// the variable and not the package...
	server.SetActives(http.NewActiveRequests())
	server, err := server.NewServer(ds, sys, configstore, acctstore, "json",
		false, 10, 10, 4, 4, 0, 0, false, false, false, true,
		server.ProfOff, false)
	if err != nil {
		logging.Errorp(err.Error())
		os.Exit(1)
	}
	prepareds.PreparedsReprepareInit(ds, sys)
	constructor.Init(nil)

	server.SetKeepAlive(1 << 10)
	server.SetMaxIndexAPI(datastore.INDEX_API_MAX)

	mockServer.server = server
	mockServer.acctstore = acctstore
	mockServer.dstore = ds

	GlobalServer.Store(mockServer)
}

type scanConfigImpl struct {
}

func (this *scanConfigImpl) ScanConsistency() datastore.ScanConsistency {
	return datastore.SCAN_PLUS
}

func (this *scanConfigImpl) ScanWait() time.Duration {
	return 0
}

func (this *scanConfigImpl) ScanVectorSource() timestamp.ScanVectorSource {
	return &http.ZeroScanVectorSource{}
}

//export RunQuery
func RunQuery(q *C.char) *C.char {

	statement := C.GoString(q)
	logger, _ := log_resolver.NewLogger("golog")
	logging.SetLogger(logger)
	runtime.GOMAXPROCS(1)

	mockServer := GlobalServer.Load().(*MockServer)

	namespace := "json"

	var metrics value.Tristate
	scanConfiguration := &scanConfigImpl{}

	pretty := value.TRUE

	mr := &MockResponse{
		results:  []interface{}{},
		warnings: []errors.Error{},
		done:     make(chan bool),
	}
	query := &MockQuery{
		response: mr,
	}
	server.NewBaseRequest(&query.BaseRequest)
	query.SetStatement(statement)
	// query.SetNamedArgs(namedArgs)
	// query.SetPositionalArgs(positionalArgs)
	query.SetNamespace(namespace)
	query.SetReadonly(value.FALSE)
	query.SetMetrics(metrics)
	query.SetSignature(value.TRUE)
	query.SetPretty(pretty)
	query.SetScanConfiguration(scanConfiguration)

	// defer mockServer.doStats(query)

	if !mockServer.server.ServiceRequest(query) {
		println("TIMEOUT")
	}

	// wait till all the results are ready
	<-mr.done

	// res := fmt.Sprintf("%v", mr.results)
	res, _ := json.Marshal(mr.results)
	return C.CString(string(res))
}

func main() {

}

# mullet - couchbase in the front, party in the back

We also need a moustache.

## Building

You need go 1.11 installed, right now it looks for it at `/usr/local/opt/go@1.11/bin/go` but we can make this
tweakable easily in the `build.rs`.

It only works on osx right now, because we hardcode some link stuff. You can build with `cargo build` but really
you want to run it with

`cargo run -- "select * from default:game"`

Http server and all that will be added later, right now this is a POC that we can embed the query engine. boom.
Also we need to silence cbauth, likely with a stub listener running in this mock too.

Here is some proof, since you didn't believe it, right? right???

```
~/couchbase/code/rust/mullet$ cargo run -- "select * from default:game"
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/mullet 'select * from default:game'`
Query: "select * from default:game"
2019/09/20 22:36:38 cbauth: key: fts/remoteClients registered for tls config updates
2019/09/20 22:36:38 cfg_metakv: RunObserveChildren, err: Get /_metakv/fts/cbgt/cfg/?feed=continuous: Unable to initialize cbauth's revrpc: cbauth environment variable CBAUTH_REVRPC_URL is not set
_time=2019-09-20T22:36:38.095+02:00 _level=INFO _msg=Unable to initialize functions cache monitor Could not access functions change counter because Put /_metakv/query/functions_cache/counter: Unable to initialize cbauth's revrpc: cbauth environment variable CBAUTH_REVRPC_URL is not set - cause: Put /_metakv/query/functions_cache/counter: Unable to initialize cbauth's revrpc: cbauth environment variable CBAUTH_REVRPC_URL is not set
_time=2019-09-20T22:36:38.096+02:00 _level=INFO _msg=New susbscription /query/settings/ done:Get /_metakv/query/settings/?feed=continuous: Unable to initialize cbauth's revrpc: cbauth environment variable CBAUTH_REVRPC_URL is not set
Result: "[{\"game\":{\"id\":\"damien\",\"roles\":[\"beta\"],\"score\":10,\"type\":\"player\"}},{\"game\":{\"id\":\"dustin\",\"score\":10,\"type\":\"player\"}},{\"game\":{\"id\":\"junyi\",\"roles\":[\"map-editor\",\"GM\"],\"score\":100,\"type\":\"player\"}},{\"game\":{\"id\":\"marty\",\"roles\":[\"beta\",\"alpha\"],\"score\":8,\"type\":\"player\"}},{\"game\":{\"id\":\"steve\",\"roles\":[\"emp\"],\"score\":1,\"type\":\"player\"}}]"
```
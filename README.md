# mullet - couchbase in the front, party in the back

We also need a moustache.

## Building

You need go 1.11 installed, right now it looks for it at `/usr/local/opt/go@1.11/bin/go` but we can make this
tweakable easily in the `build.rs`.

It only works on osx right now, because we hardcode some link stuff. You can build with `cargo build` but really
you want to run it with

`cargo run`

and then you can curl a query:

```
$ curl -H "Content-Type: application/json" -d "{\"statement\": \"select 1=1\"}" http://127.0.0.1:9093/query/service
[{"$1":true}]
```

You can also use cbq:

```
$ cbq -e 127.0.0.1:9093
 No input credentials. In order to connect to a server with authentication, please provide credentials.
 Connected to : http://127.0.0.1:9093/. Type Ctrl-D or \QUIT to exit.

cbq> select * from default:game;
[{"game":{"id":"damien","roles":["beta"],"score":10,"type":"player"}},{"game":{"id":"dustin","score":10,"type":"player"}},{"game":{"id":"junyi","roles":["map-editor","GM"],"score":100,"tcbq>
```
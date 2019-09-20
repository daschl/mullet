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
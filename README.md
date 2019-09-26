# mullet - couchbase in the front, party in the back

We also need a moustache.

## Building

You need go 1.11 installed, right now it looks for it at `/usr/local/opt/go@1.11/bin/go` but we can make this
tweakable easily in the `build.rs`.

It only works on osx right now, because we hardcode some link stuff. You can build with `cargo build` but really
you want to run it with

`cargo run -- -c sample_config.json`

You can modify the sample config that ships in this repository, but for now pretty much everything that's in the
sample is supported.

## Usage

Once started, it will print some debug information, most importantly which ports are used:

```
Sep 26 15:19:14.434 DEBG Loaded configuration MulletClusterConfig { nodes: [MulletNodeConfig { services: [Query] }, MulletNodeConfig { services: [Query] }], low_port: 9000 }
Sep 26 15:19:14.435 DEBG Starting Cluster with 2 nodes at port range start 9000
Sep 26 15:19:14.435 DEBG Starting Node 0 at port offset 0
Sep 26 15:19:14.435 DEBG Starting Manager Service at port 9001
Sep 26 15:19:14.437 DEBG Starting Query Service at port 9003
```

This gives you a clue that the cluster manager is running at `127.0.0.1:9001` for one of the nodes and the query service
at port `9003`. For now each node in the list gets 10 port range, but this might change in the future so don't depend
on it at all.
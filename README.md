## Experimentation repo for ZeroMQ and ProtoBuf in Rust

### Usage
1) Build the executable with `cargo build`
2) Start the server with `target/debug/pingpong server`
3) Start the client with `target/debug/pingpong client`

You should a loop of pings sent and pongs received by the client:
```
Sending -> "PING"
Received <- "PONG"
Sending -> "PING"
Received <- "PONG"
```
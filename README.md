# Redis server in Rust

This is a simple Redis cache server written in rust.

## How to use ?

1. Ensure you have Rust and Cargo installed locally.
2. Run `./spawn_redis_server.sh` to run you Redis servers.


## Commands

- [x] ping
- [x] echo
- [x] get
- [x] set
- [ ] others ==> `todo!()`

## Project structure

This project is divided into 2 main parts, bin `main.rs` and lib `lib.rs`.

### `main.rs`

It has only one function to do, and that is to execute the `rudis::startup::run()` function.

### `lib.rs`

Our library is partitioned into `startup`, `cache.rs`, `operations` and `resp`.

#### startup

This the startup point of our server it is responsible for running the listener, handling connection, initializing the cache and priority queue, and spawns a worker thread that removes temporary entries pushed into cache.

#### cache

This is where our `Cache` and `PriorityQueue` are defined.

#### operations

This is where our operations are implemented right now `rudis` supports only 4 operations `ping`, `echo`, `get` and `set`.

#### resp

This is where everything related to [RESP](https://redis.io/docs/reference/protocol-spec/) is implemented, parser buffer and errors.

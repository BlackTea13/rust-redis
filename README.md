# Rust-Redis

A redis clone built in Rust. It supports the RESP2 protocol only and a limited number of commands. The project uses [Async Rust](https://rust-lang.github.io/async-book/) for performance multi-client handling. 

### Supported Commands

- SELECT
- PING
- GET
- SET
- EXISTS
- LPUSH
- RPUSH
- BLPOP
- RLPOP

See the command spec and documentation [here](https://redis.io/docs/latest/commands/).




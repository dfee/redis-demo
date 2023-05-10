# rust-demo

currently just an implementation of a redis client (supporting `get`/`set`) based on the
[tokio tutorial](https://tokio.rs/tokio/tutorial).

three files:
1. [src/lib.rs](./src/lib.rs): hosts an enum (sum type) that the client / server communicate with.
2. [src/bin/client.rs](./src/bin/client.rs): executes a set then get command
3. [src/bin/server.rs](./src/bin/server.rs): the redis runtime


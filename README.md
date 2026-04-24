# rust-todo-list
A pet project to practice and learn rust. This starts from a very basic input output binary file to adding more novle features.

# How to setup
- Clone the repo
```sh
git clone https://github.com/ningthoujamSwamikumar/rust-todo-list.git
```

# Run cli app
> The whole app is a CLI application. The app takes the cli argument and perform the action. The app stores the data in a file 
```bash
# Run cli app i.e. src/bin/cli_app.rs
cd rust-todo-list && cargo run --bin cli_app --help
```

# Run File based client-server app
> The app has a server which we need to run first and then run the client to perform the operations on the server. The client is still a cli app. The server still uses file as the storage of the data.
```bash
# Run server
cd rust-todo-list && cargo run --bin file_server
```

And in separate terminal window
```sh
# Run client (cli)
cd rust-todo-list && cargo run --bin file_client --help
```

# Run Db based client-server app
> The app has a server which we need to run first and then run the client to perform the operations on the server. The client is still a cli app. The server uses a database as the storage of the data.

```bash
# Start a postgres docker for the database
docker run --name todo-postgres \
  -e POSTGRES_HOST_AUTH_METHOD=trust \
  -p 5432:5432 \
  -d postgres:latest
```
```bash
# Run server
cd rust-todo-list && cargo run --bin db_server
```
And in separate terminal window
```sh
# Run client (cli)
cd rust-todo-list && cargo run --bin db_client --help
```

# Concepts
- Cli argument parsing using `clap` [cli_parser](src/cli_parser.rs)
- File io or read/write
- Tcp server-client using `tokio`
- Json serialization/deserialization using `serde` and `serde_json`
- Tcp byte stream framing, and efficient read buffering using `bytes`
- Distribution of responsibility over Network handler (server) and Processor (worker).
- Message passing over threads using `mpsc` and `oneshot`
- Async rust with `tokio`
- Db connection and queries using `sqlx`

# TODO List
- [x] MVP - take an input and add it to list
- [x] CRUD through input commands
    - [x] Give options for actions
    - [x] Take input in the format `[Action] - [Input if any]`
    - [x] Handle ADD
    - [x] Handle DEL
    - [x] Handle UPD
- [x] Command line interface
    - [x] Redesign the code
    - [x] Parse commands
    - [x] Read/Write from/to file
- [ ] CLI GUI 
- [x] Connect Database
    - [x] Test database connection
    - [x] Segregate servers into file based server, and db based server
    - [x] Refactor todo list to accomodate database operations
    - [x] Refactor server codes to accomodate async todo methods
    - [x] Redesign to replace use of global Mutex, to scalable message passing to a worker thread
    - [x] Extract todo operations as trait and implement to different structs for db and file based apps
- [x] Server Client Arch
    - [x] Create a Server which would keep on running all the time
    - [x] Create cli client which sends packets to server
    - [x] Connect Server and client using a custom protocol
- [ ] Write Tests
- [ ] Web APIs
- [ ] Real Time Update System (Notify)
- [ ] Multi User Access/Update
- [ ] Upgrade clients to session based clients like psql interface in postgresql db

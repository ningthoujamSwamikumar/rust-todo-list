# rust-todo-list
A pet project to practice and learn rust. This starts from a very basic input output binary file to adding more novle features.

# How to setup
- Clone the repo
```sh
git clone https://github.com/ningthoujamSwamikumar/rust-todo-list.git
```
- Run cli app
```sh
# Run cli app i.e. src/bin/main.rs
cd rust-todo-list && cargo run --bin main --help
```
or Run client-server
```sh
# Run server
cd rust-todo-list && cargo run --bin server
```
And in separate terminal window
```sh
# Run client (cli)
cd rust-todo-list && cargo run --bin client --help
```

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
- [ ] Connect Database
    - [x] Test database connection
    - [x] Segregate servers into file based server, and db based server
    - [x] Refactor todo list to accomodate database operations
    - [x] Refactor server codes to accomodate async todo methods
    - [ ] Redesign to replace use of global Mutex, to scalable message passing to a worker thread
    - [ ] Extract todo operations as trait and implement to different structs for db and file based apps
- [x] Server Client Arch
    - [x] Create a Server which would keep on running all the time
    - [x] Create cli client which sends packets to server
    - [x] Connect Server and client using a custom protocol
- [ ] Write Tests
- [ ] Web APIs
- [ ] Real Time Update System (Notify)
- [ ] Multi User Access/Update
- [ ] Upgrade clients to session based clients like psql interface in postgresql db

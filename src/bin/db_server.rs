use bytes::{Buf, BytesMut};
use rust_todo_list::{
    cli_parser::Actions, db_storage::DbStorage, error::TodoError, todo_list::TodoOps,
};
use sqlx::postgres::PgPoolOptions;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres@localhost:5432/postgres")
        .await?;
    println!("Connection to postgres is successful");

    let todo_db = DbStorage::new(pool);

    // Worker need to be started first before we start listening for inbounds
    let sender_to_worker = run_worker(todo_db).await;

    // setup tcp ports and server
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let server = run_server(listener, sender_to_worker);

    //wait for either server crash or Ctrl+C
    tokio::select! {
        _ = server => {
            println!("App Crash");
        },
        _ = tokio::signal::ctrl_c()=>{
            println!("Shutdown signal received");
        }
    };

    Ok(())
}

/// starts a worker, which takes commands through a message channel and perform the action
async fn run_worker(
    mut todo: DbStorage,
) -> tokio::sync::mpsc::Sender<(
    Actions,
    tokio::sync::oneshot::Sender<Result<String, TodoError>>,
)> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<(
        Actions,
        tokio::sync::oneshot::Sender<Result<String, TodoError>>,
    )>(32);

    tokio::spawn(async move {
        let mut retrievals: Vec<String> = Vec::new();

        loop {
            retrievals.clear(); //cleared to handle new action

            let Some((action, oneshot_sender)) = rx.recv().await else {
                println!("Message channel has been closed. Exiting ...");
                return;
            };

            let result = match action {
                Actions::Add { task } => todo.add(task).await.map(|_| String::from("Ok")),
                Actions::Delete { id } => todo.delete(id).await.map(|_| String::from("Ok")),
                Actions::Show { id } => {
                    if let Some(i) = id {
                        todo.get(i as i32)
                            .await
                            .map(|value| format!("Ok\n{}", value))
                    } else {
                        todo.get_all().await.map(|value| format!("Ok\n{}", value))
                    }
                }
                Actions::Update { id, task } => {
                    todo.update(id, task).await.map(|_| String::from("Ok"))
                }
                Actions::Clear => todo.clear().await.map(|_| String::from("Ok")),
            };

            if let Err(_) = oneshot_sender.send(result) {
                eprintln!("Server has been disconnected!");
                return;
            }
        }
    });

    tx
}

async fn run_server(
    listener: TcpListener,
    sender: tokio::sync::mpsc::Sender<(
        Actions,
        tokio::sync::oneshot::Sender<Result<String, TodoError>>,
    )>,
) -> tokio::io::Result<()> {
    loop {
        // Accepts a tcp connection
        let (tcp_stream, socket_addr) = listener.accept().await?;
        println!("Connection recieved from client at - {}", socket_addr);

        let sender_copy = sender.clone();

        tokio::spawn(async move {
            process(tcp_stream, sender_copy).await;
        });
    }
}

async fn process(
    mut tcp_stream: TcpStream,
    sender: tokio::sync::mpsc::Sender<(
        Actions,
        tokio::sync::oneshot::Sender<Result<String, TodoError>>,
    )>,
) {
    let (tx, rx) = tokio::sync::oneshot::channel::<Result<String, TodoError>>();

    let mut buf: BytesMut = BytesMut::new();
    let received_action;

    //framing the receiving bytes
    loop {
        match tcp_stream.read_buf(&mut buf).await {
            Err(e) => {
                eprintln!("Error reading bytes to buffer:\n{:?}", e);
                return;
            }
            Ok(0) => {
                println!("Connection closed.");
                return;
            }
            _ => {}
        };

        //find the delimitor of the frame
        if let Some(n) = buf.iter().position(|&b| b == 0) {
            //take the frame from the bytesmut var i.e. buf and write it to received.
            let frame = buf.split_to(n);
            buf.advance(1); //skip the null character '\0'
            //we can directly convert to json from byte slice, without converting first to string
            received_action = serde_json::from_slice::<Actions>(&frame);
            break;
        }
    }

    let mut response = match received_action {
        Ok(action) => {
            if sender.send((action, tx)).await.is_err() {
                eprintln!("Failed to pass message to worker thread!");
                String::from("Internal Error - Failed to connect to worker thread!")
            } else {
                match rx.await {
                    Ok(res) => match res {
                        Ok(r) => r,
                        Err(e) => e.to_string(),
                    },
                    Err(e) => e.to_string(),
                }
            }
        }
        Err(e) => {
            eprintln!("Error deserialization:\n{:?}", e);
            format!("Failed deserialization\n{:?}", e)
        }
    };

    //Append delimitor
    response.push('\0');
    //write back the response
    if let Err(e) = tcp_stream.write_all(response.as_bytes()).await {
        eprintln!("Error writing: {:?}", e);
        return;
    }
    println!("Response sent: {}", response);
}

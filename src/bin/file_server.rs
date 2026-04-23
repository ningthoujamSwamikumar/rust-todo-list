use std::{path::Path, sync::Arc};

use rust_todo_list::{action_handler, cli_parser, todo_list};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let path = Path::new("target/.todo_list.json");
    let list = match todo_list::TodoList::from_file(&path) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error creating todo_list from file:\n{:?}", e);
            todo_list::TodoList::new()
        }
    };

    let arc_list = Arc::new(tokio::sync::Mutex::new(list));

    let server = run_server(listener, arc_list.clone());

    //wait for either server crash or Ctrl+C
    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c()=>{
            println!("Shutdown signal received");
        }
    };

    //save on shutdown
    match arc_list.lock().await.write_file(&path) {
        Ok(()) => println!("List saved to file"),
        Err(e) => eprintln!("Failed to saved the list to file - \n{:?}", e),
    };

    Ok(())
}

async fn run_server(
    listener: TcpListener,
    arc_list: Arc<tokio::sync::Mutex<todo_list::TodoList>>,
) -> tokio::io::Result<()> {
    loop {
        // Accepts a tcp connection
        let (tcp_stream, socket_addr) = listener.accept().await?;
        println!("Connection recieved from client at - {}", socket_addr);

        let list = arc_list.clone();

        tokio::spawn(async move {
            process(tcp_stream, list).await;
        });
    }
}

async fn process(mut tcp_stream: TcpStream, list: Arc<tokio::sync::Mutex<todo_list::TodoList>>) {
    let mut received = String::new();
    let (read_half, mut write_half) = tcp_stream.split();
    //wrap the stream in a buffered reader
    let mut reader = BufReader::new(read_half);
    loop {
        received.clear(); // Clear the string for the next loop

        match reader.read_line(&mut received).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading: {:?}", e);
                return;
            }
        };

        println!("received: {}", received);

        let received_action = serde_json::from_str::<cli_parser::Actions>(&received);
        let response = match received_action {
            Ok(action) => {
                let mut retrievals = Vec::<String>::new();
                let mut list = list.lock().await;
                match action_handler(action, &mut list, &mut retrievals).await {
                    Ok(()) => {
                        let mut values = retrievals.join(", ");
                        if retrievals.len() > 1 {
                            values.insert_str(0, "[ ");
                            values.push_str(" ]\n");

                            values
                        } else if retrievals.len() == 1 {
                            values
                        } else {
                            String::from("Ok")
                        }
                    }
                    Err(e) => e.to_string(),
                }
            }
            Err(e) => format!(
                "Something went wrong in deserialization of action!\n{:?}",
                e
            ),
        };

        //write back the response
        if let Err(e) = write_half.write_all(response.as_bytes()).await {
            eprintln!("Error writing: {:?}", e);
            return;
        }
        println!("Response sent: {}", response);
    }
}

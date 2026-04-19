use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use rust_todo_list::{
    action_handler,
    cli_parser::{self, Actions},
    todo_list,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let path = Path::new("target/.todo_list.json");
    let list = match todo_list::TodoList::from_file(&path) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error creating todo_list from file:\n{:?}", e);
            todo_list::TodoList::new()
        }
    };

    let arc_list = Arc::new(Mutex::new(list));

    let server = run_server(listener, arc_list.clone());

    //wait for either server crash or Ctrl+C
    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c()=>{
            println!("Shutdown signal received");
        }
    };

    //save on shutdown
    match arc_list.lock().unwrap().write_file(&path) {
        Ok(()) => println!("List saved to file"),
        Err(e) => eprintln!("Failed to saved the list to file - \n{:?}", e),
    };

    Ok(())
}

async fn run_server(
    listener: TcpListener,
    arc_list: Arc<Mutex<todo_list::TodoList>>,
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

async fn process(mut tcp_stream: TcpStream, list: Arc<Mutex<todo_list::TodoList>>) {
    let mut buf = vec![0; 1024];
    loop {
        let n = match tcp_stream.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading: {:?}", e);
                return;
            }
        };

        let recieved = String::from_utf8_lossy(&buf[0..n]);
        println!("received: {}", recieved);

        let received_action = serde_json::from_str::<cli_parser::Actions>(&recieved);
        let response = match received_action {
            Ok(action) => {
                let mut retrievals = Vec::<String>::new();
                let mut list = list.lock().unwrap();
                match action_handler(&action, &mut list, &mut retrievals) {
                    Ok(()) => {
                        let mut result = String::new();
                        match action {
                            Actions::Show { index: None } => {
                                let values = retrievals.join(", ");
                                result.push_str("[ ");
                                result.push_str(&values);
                                result.push_str(" ]\n");
                            }
                            Actions::Show { index: Some(_) } => {
                                result.push_str(&retrievals.join(", "))
                            }
                            _ => result.push_str("Ok"),
                        };
                        result
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
        if let Err(e) = tcp_stream.write_all(response.as_bytes()).await {
            eprintln!("Error writing: {:?}", e);
            return;
        }
        println!("Response sent: {}", response);
    }
}

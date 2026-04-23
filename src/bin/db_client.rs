use bytes::BytesMut;
use clap::Parser;
use rust_todo_list::{cli_parser, error::TodoError};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), TodoError> {
    let cli = cli_parser::Cli::parse();

    let mut cli_string = serde_json::to_string(&cli.action)?;
    cli_string.push('\0');

    let mut tcp_stream = TcpStream::connect("127.0.0.1:6379").await?;
    println!("Connected to server");

    tcp_stream.write_all(cli_string.as_bytes()).await?;
    println!("Sent: {}", cli_string);

    let mut buf = BytesMut::new();
    loop {
        match tcp_stream.read_buf(&mut buf).await {
            Ok(0) | Err(_) => {
                eprintln!("Connection closed.");
                return Err(TodoError::AccessError(
                    "Connection to server failed!".into(),
                ));
            }
            Ok(_) => {}
        };

        if let Some(n) = buf.iter().position(|&b| b == 0) {
            let frame = buf.split_to(n);
            let received = String::from_utf8_lossy(&frame);
            println!("Received: {}", received);
            break;
        };
    }
    Ok(())
}

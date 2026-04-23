use clap::Parser;
use rust_todo_list::cli_parser;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = cli_parser::Cli::parse();

    let mut cli_string = serde_json::to_string(&cli.action)?;
    cli_string.push('\n');

    let mut tcp_stream = TcpStream::connect("127.0.0.1:6379").await?;
    println!("Connected to server");

    tcp_stream.write_all(cli_string.as_bytes()).await?;
    println!("Sent: {}", cli_string);

    let mut buf = vec![0; 1024];
    let n = tcp_stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}

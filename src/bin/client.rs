use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};


#[tokio::main]
async fn main()->io::Result<()>{
    let mut tcp_stream = TcpStream::connect("127.0.0.1:6379").await?;
    println!("Connected to server");

    tcp_stream.write_all(b"PING").await?;
    println!("Sent: PING");

    let mut buf = vec![0; 1024];
    let n = tcp_stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // Accepts a tcp connection
        let (tcp_stream, socket_addr) = listener.accept().await?;
        println!("Connection recieved from client at - {}", socket_addr);

        tokio::spawn(async move {
            process(tcp_stream).await;
        });
    }
}

async fn process(mut tcp_stream: TcpStream) {
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

        println!("Recieved: {}", String::from_utf8_lossy(&buf[0..n]));

        //echo back what was received
        if let Err(e) = tcp_stream.write_all(&buf[0..n]).await {
            eprintln!("Error writing: {:?}", e);
            return;
        }
        println!("Sent back the same message.");
    }
}

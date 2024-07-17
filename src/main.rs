use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(socket: TcpStream) {
    println!("Handling client: {:?}", socket);

    let (mut reader, mut writer) = tokio::io::split(socket);

    let mut buf = vec![0; 1024];
    loop {
        let n = match reader.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => {
                println!("read {} bytes", n);
                n
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        if let Err(e) = writer.write_all(&buf[..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener =TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((socket, addr)) = listener.accept().await {
        println!("Accepted connection from: {}", addr);
        tokio::spawn(handle_client(socket));
    }

    Ok(())
}

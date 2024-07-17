use log::{debug, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(socket: TcpStream) {
    debug!("Handling client: {:?}", socket);

    let (mut reader, mut writer) = tokio::io::split(socket);

    let mut buf = vec![0; 1024];
    loop {
        let n = match reader.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => {
                debug!("read {} bytes", n);
                n
            }
            Err(e) => {
                warn!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        if let Err(e) = writer.write_all(&buf[..n]).await {
            warn!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    debug!("Starting application");
    let listener =TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((socket, addr)) = listener.accept().await {
        debug!("Accepted connection from: {}", addr);
        tokio::spawn(handle_client(socket));
    }

    Ok(())
}

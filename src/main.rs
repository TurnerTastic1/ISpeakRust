use log::{debug, warn};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    debug!("Starting application");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = split(socket);

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let _bytes_read = match reader.read_line(&mut line).await {
                    Ok(bytes_read) if bytes_read == 0 => {
                        debug!("connection closed");
                        break;
                    }
                    Ok(bytes_read) => {
                        debug!("read {} bytes", bytes_read);
                        bytes_read
                    }
                    Err(e) => {
                        warn!("failed to read from socket; err = {:?}", e);
                        break;
                    }
                };

                if let Err(e) = writer.write_all(line.as_bytes()).await {
                    warn!("failed to write to socket; err = {:?}", e);
                    break;
                }

                line.clear();
            }
        });
    }

    Ok(())
}

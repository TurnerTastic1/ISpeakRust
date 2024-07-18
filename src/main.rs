use log::{debug, warn};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    debug!("Starting application");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = split(socket);

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(bytes_read) if bytes_read == 0 => {
                                debug!("connection closed");
                                break;
                            }
                            Ok(bytes_read) => {
                                debug!("read {} bytes", bytes_read);
                                tx.send((line.clone(), addr)).unwrap();
                                line.clear();
                            }
                            Err(e) => {
                                warn!("failed to read from socket; err = {:?}", e);
                                break;
                            }
                        }
                    }

                    result = rx.recv() => {
                        let (msg, recv_addr) = result.unwrap();

                        if addr != recv_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                            // if let Err(e) = writer.write_all(msg.as_bytes()).await {
                            //     warn!("failed to write to socket; err = {:?}", e);
                            //     break;
                            // }
                        }
                    }
                }
            }
        });
    }
}

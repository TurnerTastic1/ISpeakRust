use log::{debug, warn};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::{TcpListener};
use tokio::sync::broadcast;
use crate::application::error::errors::ApplicationError;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server { listener }
    }

    pub async fn initialize() -> Result<Self, ApplicationError> {
        let listener = match TcpListener::bind("127.0.0.1:8080").await {
            Ok(listener) => listener,
            Err(e) => {
                warn!("Failed to bind to port 8080: {:?}", e);
                return Err(ApplicationError::Custom("Failed to bind to port 8080".parse().unwrap()));
            }
        };

        Ok(Server::new(listener))
    }

    pub async fn start_server(&self) -> Result<(), ApplicationError> {
        debug!("Starting server");

        let (tx, _rx) = broadcast::channel(10);

        loop {
            let (mut socket, addr) = self.listener.accept().await.unwrap();

            debug!("Accepted connection from {:?}", addr);

            let tx = tx.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                if let Err(e) = socket.write_all(b"Please enter your username:\n").await {
                    warn!("failed to write to socket; err = {:?}", e);
                    return ApplicationError::Custom("Failed to write to socket".parse().unwrap());
                }

                let (reader, mut writer) = split(socket);
                let mut reader = BufReader::new(reader);

                let username = read_line(&mut reader).await.unwrap().trim().to_string();
                debug!("Username: {}", username);

                loop {
                    tokio::select! {
                        result = read_line(&mut reader) => {
                            match result {
                                Ok(line) => {
                                    let msg = format!("\x1b[32m{}\x1b[0m: {}", username, line);

                                    tx.send((msg.clone(), addr)).unwrap();
                                }
                                Err(_) => {
                                    warn!("Failed to read from socket {}", addr);
                                    return ApplicationError::Custom("Failed to read from socket".parse().unwrap());
                                }
                            }
                        }
                        result = rx.recv() => {
                            let (msg, recv_addr) = result.unwrap();

                            if addr != recv_addr {
                                if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                    warn!("Failed to write from socket {} - Error: {}", addr, e);
                                    return ApplicationError::Custom("Failed to write from socket".parse().unwrap());
                                }
                            }
                    }
                    }
                }
            });
        }
    }
}

async fn read_line<R>(reader: &mut R) -> Result<String, ApplicationError>
where
    R: AsyncBufReadExt + Unpin,
{
    let mut line = String::new();
    match reader.read_line(&mut line).await {
        Ok(bytes_read) if bytes_read == 0 => {
            // Connection closed
            Ok(String::new())
        }
        Ok(_) => Ok(line),
        Err(e) => Err(ApplicationError::Custom(format!("Failed to read line: {:?}", e))),
    }
}
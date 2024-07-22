use log::{debug, warn};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::{TcpListener};
use tokio::sync::broadcast;
use crate::application::model::error::{ApplicationError, ErrorSeverity};
use crate::application::model::message::Message;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server { listener }
    }

    pub async fn initialize() -> Result<Self, ApplicationError> {
        let listener = match TcpListener::bind("0.0.0.0:8080").await {
            Ok(listener) => listener,
            Err(e) => {
                warn!("Failed to bind to port 8080: {:?}", e);
                return Err(ApplicationError::new(
                    "Failed to bind to port 8080",
                    Some(Box::new(e)),
                    ErrorSeverity::ERROR,
                ));
            }
        };

        Ok(Server::new(listener))
    }

    pub async fn start_server(&self) -> Result<(), ApplicationError> {
        debug!("Starting server");

        let (tx, _rx) = broadcast::channel(10);

        loop {
            let (mut socket, addr) = match self.listener.accept().await {
                Ok((socket, addr)) => (socket, addr),
                Err(e) => {
                    warn!("Failed to accept connection: {:?}", e);
                    return Err(ApplicationError::new(
                        "Failed to accept connection",
                        Some(Box::new(e)),
                        ErrorSeverity::ERROR,
                    ));
                }
            };

            debug!("Accepted connection from {:?}", addr);

            let tx = tx.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                if let Err(e) = socket.write_all(b"Please enter your username:\n").await {
                    warn!("failed to write to socket; err = {:?}", e);
                    return ApplicationError::new("Failed to write to socket", None, ErrorSeverity::CRITICAL);
                }

                let (reader, mut writer) = split(socket);
                let mut reader = BufReader::new(reader);

                let username = match read_line(&mut reader).await {
                    Ok(line) => {
                        if line.message.trim().is_empty() {
                            return ApplicationError::new(
                                "Username cannot be empty",
                                None,
                                ErrorSeverity::WARN,
                            );
                        }
                        line.message.trim().to_string()
                    }
                    Err(e) => {
                        warn!("{}", e.message);
                        return e;
                    }
                };
                debug!("Username: {}", username);

                loop {
                    tokio::select! {
                        result = read_line(&mut reader) => {
                            match result {
                                Ok(line) => {
                                    let msg = format!("\x1b[32m{}\x1b[0m: {}", username, line.message);

                                    tx.send((msg.clone(), addr)).unwrap();
                                }
                                Err(e) => {
                                    warn!("{}" ,e.message);
                                    return e;
                                }
                            }
                        }
                        result = rx.recv() => {
                            let (msg, recv_addr) = result.unwrap();

                            if addr != recv_addr {
                                if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                    warn!("Failed to write from socket {} - Error: {}", addr, e);
                                    return ApplicationError::new(
                                        "Failed to write from socket",
                                        None,
                                        ErrorSeverity::ERROR,
                                    );
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}

async fn read_line<R>(reader: &mut R) -> Result<Message, ApplicationError>
where
    R: AsyncBufReadExt + Unpin,
{
    let mut line = String::new();
    match reader.read_line(&mut line).await {
        Ok(bytes_read) if bytes_read == 0 => {
            // Connection closed
            Ok(Message::new("".to_string()))
        }
        Ok(_) => Ok(
            Message::new(
                line
            )
        ),
        Err(e) => Err(
            ApplicationError::new(
                "Failed to read line",
                Some(Box::new(e)),
                ErrorSeverity::ERROR,
            )
        ),
    }
}

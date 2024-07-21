use log::{debug, warn};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
use tokio::net::{TcpListener};
use tokio::sync::broadcast;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server { listener }
    }

    pub async fn initialize() -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        Ok(Server::new(listener))
    }

    pub async fn start_server(&self) -> Result<(), Box<dyn std::error::Error>> {
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
                    return Err::<(), std::io::Error>(e.into());
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
                                Err(e) => {
                                    warn!("Failed to read from socket {} - Error: {}", addr, e);
                                    return Err(e.into());
                                }
                            }
                        }
                        result = rx.recv() => {
                            let (msg, recv_addr) = result.unwrap();

                            if addr != recv_addr {
                                if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                    warn!("Failed to write from socket {} - Error: {}", addr, e);
                                    return Err(e.into());
                                }
                            }
                    }
                    }
                }
            });
        }
    }
}

async fn read_line<R>(reader: &mut R) -> Result<String, std::io::Error>
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
        Err(e) => Err(e),
    }
}
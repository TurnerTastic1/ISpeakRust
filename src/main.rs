use std::collections::HashMap;
use log::{debug, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt, split};
use tokio::net::{TcpListener, TcpStream};

struct Session {
    id: u64,
    socket: TcpStream
}

impl Session {
    async fn new(id: u64, socket: TcpStream) -> Self {
        debug!("Created session with id: {}", id);
        Session { id, socket }
    }

    async fn run(&mut self) {
        let (mut reader, mut writer) = split(&mut self.socket);

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    debug!("Starting application");
    let listener =TcpListener::bind("127.0.0.1:8080").await?;

    let mut _sessions: HashMap<u64, Session> = HashMap::new();
    let mut next_session_id = 0usize;

    while let Ok((socket, addr)) = listener.accept().await {
        debug!("Accepted connection from: {}", addr);

        let session = Session::new(next_session_id as u64, socket).await;

        tokio::spawn(async move {
            let mut session = session;
            session.run().await;
        });

        next_session_id += 1;
    }

    Ok(())
}

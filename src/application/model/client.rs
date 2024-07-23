use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    pub socket: TcpStream,
    pub addr: SocketAddr
}

impl Client {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        Client {
            socket,
            addr
        }
    }
}

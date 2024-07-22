//
// #[cfg(test)]
// mod tests {
//     use std::net::{IpAddr, Ipv4Addr};
//     use log::warn;
//     use tokio::net::unix::SocketAddr;
//     use tokio::sync::broadcast;
//     use tokio_test;
//     use crate::application::server::handle_client;
//
//     #[tokio::test]
//     async fn test_client_handler() {
//         let reader = tokio_test::io::Builder::new()
//             .read(b"Hi there\r\n")
//             .read(b"How are you doing?\r\n")
//             .build();
//         let writer = tokio_test::io::Builder::new()
//             .write(b"Thanks for your message.\r\n")
//             .write(b"Thanks for your message.\r\n")
//             .build();
//
//         let (mock_tx, _rx) = broadcast::channel(10);
//
//         let _ = handle_client(reader, writer, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080), mock_tx).await;
//     }
// }

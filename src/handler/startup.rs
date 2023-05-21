use crate::Result;

use tokio::net::TcpListener;

use super::connection_handler::handle_connection;

/// A function that runs our event loop
pub async fn run(socket_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(socket_addr).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => tokio::spawn(handle_connection(stream)),
            Err(e) => return Err(e.into()),
        };
    }
}

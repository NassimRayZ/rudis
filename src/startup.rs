use std::sync::{Arc, RwLock};

use crate::{cache::Cache, operations::process};
use tokio::net::TcpListener;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const BUFFER_LENGTH: usize = 1024;

/// A function that runs our event loop
pub async fn run(socket_addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(socket_addr).await?;
    let cache = Arc::new(RwLock::new(Cache::new()));
    loop {
        match listener.accept().await {
            Ok((stream, _)) => tokio::spawn(handle_connection(stream, cache.clone())),
            Err(e) => return Err(e.into()),
        };
    }
}

pub(super) async fn handle_connection(
    mut stream: TcpStream,
    cache: Arc<RwLock<Cache>>,
) -> std::io::Result<()> {
    let mut recv = [0u8; BUFFER_LENGTH];
    loop {
        let num_bytes = stream.read(&mut recv).await?;
        if num_bytes == 0 {
            println!("Connection closed");
            break Ok(());
        }
        let send = process(&mut recv, &cache);
        stream.write(&send).await?;
    }
}

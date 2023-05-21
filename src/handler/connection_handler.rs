use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::operations::process;

use super::BUFFER_LENGTH;

pub(super) async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut recv = [0u8; BUFFER_LENGTH];
    loop {
        let num_bytes = stream.read(&mut recv).await?;
        if num_bytes == 0 {
            println!("Connection closed");
            break Ok(());
        }
        let send = process(&mut recv);
        stream.write(&send).await?;
    }
}

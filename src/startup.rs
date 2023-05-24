use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::cache::PriorityQueue;
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
    let pq = Arc::new(PriorityQueue::new());
    {
        let cache = cache.clone();
        let pq = pq.clone();
        std::thread::spawn(move || handle_cache(cache, pq));
    }
    loop {
        match listener.accept().await {
            Ok((stream, _)) => tokio::spawn(handle_connection(stream, cache.clone(), pq.clone())),
            Err(e) => return Err(e.into()),
        };
    }
}

pub(super) async fn handle_connection(
    mut stream: TcpStream,
    cache: Arc<RwLock<Cache>>,
    pq: Arc<PriorityQueue>,
) -> std::io::Result<()> {
    let mut recv = [0u8; BUFFER_LENGTH];
    loop {
        let num_bytes = stream.read(&mut recv).await?;
        if num_bytes == 0 {
            println!("Connection closed");
            break Ok(());
        }
        let send = process(&mut recv, &cache, &pq);
        stream.write(&send).await?;
    }
}

fn handle_cache(cache: Arc<RwLock<Cache>>, pq: Arc<PriorityQueue>) {
    let mut guard = pq
        .entries
        .lock()
        .expect("Failed to aquire `Mutex` from receiver");
    let mut dur = Duration::from_secs(0);
    loop {
        if let Some(entry) = guard.peek() {
            dur = entry.instant - Instant::now();
            if dur <= Duration::from_secs(0) {
                let entry = guard.pop().unwrap();
                cache
                    .write()
                    .expect("Failed to aquire `RwLock` for writing")
                    .remove(&entry.key);
            }
        }
        if guard.is_empty() {
            guard = pq
                .filled
                .wait(guard)
                .expect("Failed while waiting for `PriorityQueue` to be filled");
            continue;
        }
        guard = pq
            .filled
            .wait_timeout(guard, dur)
            .expect("Failed while waiting with timeout for `PriorityQueue` to be filled")
            .0;
    }
}

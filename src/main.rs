use simple_redis::startup;
const SOCKET_ADDR: &str = "127.0.0.1:6379";

#[tokio::main]
async fn main() {
    println!("Logs will appear here!");
    startup::run(SOCKET_ADDR).await.unwrap();
}

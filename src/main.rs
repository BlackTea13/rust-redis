use tokio::io::Result;
use tokio::net::TcpListener;

mod connection;
mod frame;
mod repl;

const IP: &str = "127.0.0.1";
const PORT: &str = "6379";

#[tokio::main]
async fn main() -> Result<()> {
    let address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&address).await?;

    println!("Welcome to Robert's Redis Rumble!");
    println!("Ready for connections...");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            repl::start_repl(socket);
        });
    }
}

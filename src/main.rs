use std::net::SocketAddr;

use crate::connection::Connection;
use crate::frame::Frame;
use tokio::io::Result;
use tokio::net::{TcpListener, TcpStream};

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
        let (socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let _ = process(socket, addr).await;
        });
    }
}

async fn process(socket: TcpStream, addr: SocketAddr) -> Result<()> {
    let mut connection = Connection::new(socket);

    loop {
        let maybe_frame: Frame = connection.read_frame().await.unwrap().unwrap();
        let _ = connection.write_frame(&maybe_frame).await;
    }
}

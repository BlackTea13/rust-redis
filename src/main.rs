use std::net::SocketAddr;

use crate::connection::Connection;
use crate::database::Databases;
use crate::frame::Frame;
use crate::handler::Handler;
use std::sync::Arc;
use tokio::io::Result;
use tokio::net::{TcpListener, TcpStream};

mod command;
mod connection;
mod database;
mod frame;
mod handler;

const IP: &str = "127.0.0.1";
const PORT: &str = "6379";

#[tokio::main]
async fn main() -> Result<()> {
    let address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&address).await?;
    let databases = Arc::new(Databases::new());

    println!("Welcome to Robert's Redis Rumble!");
    println!("Ready for connections...");

    loop {
        let (socket, _) = listener.accept().await?;
        let connection = Connection::new(socket);

        let handler = Handler {
            databases: Arc::clone(&databases),
            database: databases.index(0),
            connection,
        };

        tokio::spawn(async move {
            let _ = process(handler).await;
        });
    }
}

async fn process(mut handler: Handler) -> Result<()> {
    loop {
        let maybe_frame: Frame = handler.connection.read_frame().await.unwrap().unwrap();
        let _ = handler.connection.write_frame(&maybe_frame).await;
    }
}

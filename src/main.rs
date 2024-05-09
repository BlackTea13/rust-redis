use crate::connection::Connection;
use crate::database::Databases;
use crate::frame::Frame;
use crate::handler::Handler;
use bytes::Bytes;
use command::Command;
use mini_redis::Result;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

mod command;
mod connection;
mod database;
mod frame;
mod handler;
mod parse;

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
            connection: connection,
            sender: databases.senders[0].clone(),
        };

        tokio::spawn(async move {
            let _ = process(handler).await;
        });
    }
}

async fn process(mut handler: Handler) -> Result<()> {
    loop {
        let maybe_frame: Result<Option<Frame>> = handler.connection.read_frame().await;
        let maybe_frame = maybe_frame?;
        let frame = match maybe_frame {
            Some(frame) => frame,
            None => return Ok(()),
        };
        let command: Command = Command::from_frame(frame)?;

        command.apply(&mut handler).await?;
    }
}

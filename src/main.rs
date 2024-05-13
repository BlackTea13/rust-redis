use crate::connection::Connection;
use crate::database::{Database, Databases};
use crate::frame::Frame;
use crate::handler::{Handler, Payload};
use command::Command;
use mini_redis::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

mod command;
mod connection;
mod database;
mod frame;
mod handler;
mod parse;

const IP: &str = "127.0.0.1";
const PORT: &str = "6379";
const NUM_DB: usize = 16;

#[tokio::main]
async fn main() -> Result<()> {
    let address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&address).await?;
    let databases = Databases::new();

    let mut db_senders = Vec::new();

    for db in 0..NUM_DB {
        let (tx, rx) = mpsc::channel(32);
        db_senders.push(tx);

        let database: Arc<Database> = Arc::clone(&databases.databases[db]);

        tokio::spawn(async move {
            let _ = serve(database, rx).await;
        });
    }

    println!("Welcome to Robert's Redis Rumble!");
    println!("Ready for connections...");

    loop {
        let (socket, _) = listener.accept().await?;
        let connection = Connection::new(socket);

        let handler = Handler {
            database: Arc::clone(&databases.databases[0]),
            connection: connection,
            sender: db_senders[0].clone(),
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

        // handle select
        if let Command::SELECT(_) = Command::from_frame(frame.clone())? {
            use crate::command::Select;
            use crate::parse::Parse;

            let mut input = Parse::new(frame)?;
            let select = Select::parse_frame(&mut input);

            continue;
        }

        let command: Command = Command::from_frame(frame)?;
        let (sender, receiver) = oneshot::channel();
        let payload = Payload {
            command: command,
            sender: sender,
        };

        let _ = handler.sender.send(payload).await?;

        if let Ok(frame) = receiver.await {
            let frame: Frame = frame;
            let _ = handler.connection.write_frame(&frame).await?;
        }
    }
}

async fn serve(database: Arc<Database>, mut receiver: mpsc::Receiver<Payload>) {
    while let Some(payload) = receiver.recv().await {
        let payload: Payload = payload;

        let response = match payload.command.apply(database.clone()).await {
            Ok(frame) => frame,
            Err(_) => Frame::Error("Something went wrong...".to_string()),
        };

        let _ = payload.sender.send(response);
    }
}

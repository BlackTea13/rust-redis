use crate::command::Command;
use crate::connection::Connection;
use crate::database::{Database, Databases};
use crate::frame::Frame;
use crate::handler::{Handler, Payload};
use crate::{Result, IP, NUM_DB, PORT};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

pub async fn start_server() -> Result<()> {
    let address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&address).await?;
    let databases = Arc::new(Databases::new());

    let mut db_senders = Vec::new();

    for db in 0..NUM_DB {
        let (tx, rx) = mpsc::channel(32);
        db_senders.push(tx);

        let database: Arc<Database> = databases.databases[db].clone();

        tokio::spawn(serve(database, rx));
    }

    let db_senders: Vec<mpsc::Sender<Payload>> = db_senders;

    println!("Welcome to Robert's Redis Rumble!");
    println!("Open on {address}");
    println!("Ready for connections...");

    loop {
        let (socket, _) = listener.accept().await?;
        let connection = Connection::new(socket);

        let default_sender = db_senders[0].clone();

        let handler = Handler {
            connection,
            sender: default_sender,
        };

        let senders_clone = db_senders.clone();

        tokio::spawn(process(handler, senders_clone));
    }
}

async fn process(mut handler: Handler, senders: Vec<mpsc::Sender<Payload>>) -> Result<()> {
    loop {
        let maybe_frame: Result<Option<Frame>> = handler.connection.read_frame().await;
        let maybe_frame = maybe_frame?;
        let frame = match maybe_frame {
            Some(frame) => frame,
            None => return Ok(()),
        };

        let command: Command = match Command::from_frame(frame) {
            Ok(cmd) => cmd,
            Err(e) => {
                let response = Frame::Error(e.to_string());
                handler.connection.write_frame(&response).await?;
                continue;
            }
        };

        // handle select
        if let Command::SELECT(cmd) = command {
            handler.sender = senders[cmd.index as usize].clone();
            handler.connection.write_frame(&Frame::Simple(String::from("OK"))).await?;
            continue;
        }

        let (sender, receiver) = oneshot::channel();
        let payload = Payload { command, sender };

        handler.sender.send(payload).await?;

        if let Ok(frame) = receiver.await {
            let frame: Frame = frame;
            handler.connection.write_frame(&frame).await?;
        }
    }
}

async fn serve(database: Arc<Database>, mut receiver: mpsc::Receiver<Payload>) {
    while let Some(payload) = receiver.recv().await {
        let payload: Payload = payload;

        let db_clone = database.clone();
        tokio::spawn(async move {
            let response = payload
                .command
                .apply(db_clone)
                .await
                .unwrap_or_else(|e| Frame::Error(e.to_string()));
            let _ = payload.sender.send(response);
        });
    }
}

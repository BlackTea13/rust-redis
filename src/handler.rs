use crate::command::Command;
use crate::connection::Connection;
use crate::database::Database;
use crate::frame::Frame;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Handler {
    pub database: Arc<Database>,
    pub connection: Connection,
    pub sender: mpsc::Sender<Payload>,
}

#[derive(Debug)]
pub struct Payload {
    pub command: Command,
    pub sender: oneshot::Sender<Frame>,
}

impl Handler {}

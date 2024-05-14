use crate::command::Command;
use crate::connection::Connection;
use crate::frame::Frame;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Handler {
    pub connection: Connection,
    pub sender: mpsc::Sender<Payload>,
}

#[derive(Debug)]
pub struct Payload {
    pub command: Command,
    pub sender: oneshot::Sender<Frame>,
}

impl Handler {}

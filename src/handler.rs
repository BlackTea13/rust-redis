use crate::command::Command;
use crate::connection::Connection;
use crate::database::{Database, Databases};
use mini_redis::Result;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Handler {
    pub databases: Arc<Databases>,
    pub database: Arc<Mutex<Database>>,
    pub connection: Connection,
}

impl Handler {
    pub async fn execute(&self, command: Command) -> Result<()> {
        Ok(())
    }
}

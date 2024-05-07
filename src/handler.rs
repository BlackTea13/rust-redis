use crate::connection::Connection;
use crate::database::{Database, Databases};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Handler {
    pub databases: Arc<Databases>,
    pub database: Arc<Mutex<Database>>,
    pub connection: Connection,
}

impl Handler {}

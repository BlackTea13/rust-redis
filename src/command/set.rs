use crate::database::Database;
use crate::frame::Frame;
use crate::parse::{Parse, ParseError};
use bytes::Bytes;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
}

impl Set {
    pub async fn apply(&self, db: Arc<Database>) -> Result<Frame> {
        let _ = db.insert(&self.key, &self.value);
        let response = Frame::Simple("OK".to_string());
        Ok(response)
    }

    pub fn parse_frame(parse: &mut Parse) -> Result<Ping> {
        match parse.next_string() {
            Ok(
        }
    }
}

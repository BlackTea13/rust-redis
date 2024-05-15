use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use goms_mini_project1::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
}

impl Set {
    pub fn parse_frame(parse: &mut Parse) -> Result<Set> {
        let key = parse.next_string()?;
        let value = parse.next_bytes()?;

        Ok(Set { key, value })
    }

    pub async fn apply(&self, db: Arc<Database>) -> Result<Frame> {
        let _ = db.set(&self.key, &self.value);
        let response = Frame::Simple("OK".to_string());
        Ok(response)
    }
}

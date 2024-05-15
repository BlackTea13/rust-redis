use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use goms_mini_project1::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct RPush {
    key: String,
    elements: Vec<Bytes>,
}

impl RPush {
    pub fn parse_frame(parse: &mut Parse) -> Result<RPush> {
        let key = parse.next_string()?;
        let mut elements = Vec::new();

        if let Ok(element) = parse.next_bytes() {
            elements.push(element)
        } else {
            return Err("Error, wrong number of arguments".into());
        }

        loop {
            if let Ok(element) = parse.next_bytes() {
                elements.push(element)
            } else {
                return Ok(RPush { key, elements });
            }
        }
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let _ = database.rpush(&self.key, &self.elements)?;
        Ok(Frame::Integer(self.elements.len() as u64))
    }
}

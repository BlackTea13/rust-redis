use crate::database::Database;
use crate::frame::Frame;
use crate::parse::{Parse, ParseError};
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Exists {
    keys: Vec<String>,
}

impl Exists {
    pub fn parse_frame(parse: &mut Parse) -> Result<Exists> {
        let mut keys: Vec<String> = Vec::new();
        loop {
            match parse.next_string() {
                Ok(val) => {
                    keys.push(val);
                }
                Err(ParseError::EndOfStream) => {
                    break;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        return Ok(Exists { keys });
    }

    pub async fn apply(&self, db: Arc<Database>) -> Result<Frame> {
        let num_exists = self
            .keys
            .iter()
            .map(|k| db.exists(k))
            .filter(|k| *k == true)
            .collect::<Vec<_>>()
            .len();
        return Ok(Frame::Integer(num_exists as u64));
    }
}

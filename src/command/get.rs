use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn parse_frame(parse: &mut Parse) -> Result<Get> {
        let key = parse.next_string()?;
        Ok(Get { key })
    }

    pub async fn apply(&self, db: Arc<Database>) -> Result<Frame> {
        let response = match db.get(&self.key) {
            Some(val) => Frame::Bulk(val.clone()),
            None => Frame::Null,
        };
        Ok(response)
    }
}

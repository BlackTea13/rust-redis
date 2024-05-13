use crate::database::Database;
use crate::frame::Frame;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub async fn apply(&self, db: Arc<Database>) -> Result<Frame> {
        let response = match db.get(&self.key) {
            Some(val) => Frame::Bulk(val.clone()),
            None => Frame::Null,
        };
        Ok(response)
    }
}

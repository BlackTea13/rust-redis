use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use goms_mini_project1::Result;
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
        let result = match db.get(&self.key) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let response = match result {
            Some(val) => Frame::Bulk(val.get_value().clone()),
            None => Frame::Null,
        };

        Ok(response)
    }
}

use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct RPush {
    elements: Box<[Bytes]>,
}

impl RPush {
    pub fn parse_frame(parse: &mut Parse) -> Result<RPush> {
        unimplemented!()
    }

    pub fn apply(database: Arc<Database>) -> Result<Frame> {
        unimplemented!()
    }
}

use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct LPush {
    elements: Box<[Bytes]>,
}

impl LPush {
    pub fn parse_frame(parse: &mut Parse) -> Result<LPush> {
        unimplemented!()
    }

    pub fn apply(database: Arc<Database>) -> Result<Frame> {
        unimplemented!()
    }
}

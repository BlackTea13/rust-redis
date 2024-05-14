use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use mini_redis::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct BLPop {
    key: String,
    timeout: f32,
}

impl BLPop {
    pub fn parse_frame(parse: &mut Parse) -> Result<BLPop> {
        unimplemented!()
    }

    pub fn apply(database: Arc<Database>) -> Result<Frame> {
        unimplemented!()
    }
}

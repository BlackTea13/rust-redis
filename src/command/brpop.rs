use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use goms_mini_project1::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct BRPop {
    key: String,
    timeout: f32,
}

impl BRPop {
    pub fn parse_frame(parse: &mut Parse) -> Result<BRPop> {
        unimplemented!()
    }

    pub fn apply(database: Arc<Database>) -> Result<Frame> {
        unimplemented!()
    }
}

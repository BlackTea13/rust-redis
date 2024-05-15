use crate::parse::Parse;
use goms_mini_project1::Result;

#[derive(Debug)]
pub struct Select {
    pub index: u8,
}

impl Select {
    pub fn parse_frame(parse: &mut Parse) -> Result<Select> {
        let index = parse.next_int()?;
        Ok(Select { index: index as u8 })
    }
}

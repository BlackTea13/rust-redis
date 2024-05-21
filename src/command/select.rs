use crate::parse::Parse;
use crate::Result;

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

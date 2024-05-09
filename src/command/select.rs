use crate::frame::Frame;
use crate::handler::Handler;
use crate::parse::Parse;
use mini_redis::Result;

#[derive(Debug)]
pub struct Select {
    pub index: u8,
}

impl Select {
    pub fn parse_frames(parse: &mut Parse) -> Result<Select> {
        let index = parse.next_int()?;
        Ok(Select { index: index as u8 })
    }

    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        handler.database = handler.databases.index(self.index as usize);
        let response = Frame::Simple("OK".to_string());
        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

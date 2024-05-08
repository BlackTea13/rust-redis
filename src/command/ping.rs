use crate::frame::Frame;
use crate::handler::Handler;
use crate::parse::{Parse, ParseError};
use bytes::Bytes;
use mini_redis::Result;

#[derive(Debug, Default)]
pub struct Ping {
    message: Option<Bytes>,
}

impl Ping {
    pub fn parse_frame(parse: &mut Parse) -> Result<Ping> {
        match parse.next_bytes() {
            Ok(msg) => Ok(Ping { message: Some(msg) }),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        let response = match &self.message {
            None => Frame::Simple("PONG".to_string()),
            Some(msg) => Frame::Bulk(msg.clone()),
        };

        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

use crate::frame::Frame;
use crate::handler::Handler;
use bytes::Bytes;
use mini_redis::Result;

#[derive(Debug)]
struct Select {
    index: u8,
}

#[derive(Debug)]
struct Get {
    key: String,
}

#[derive(Debug)]
struct Set {
    key: String,
    value: Bytes,
}

#[derive(Debug)]
struct Ping {
    message: String,
}

#[derive(Debug)]
struct Exists {}

#[derive(Debug)]
pub enum Command {
    SELECT(Select),
    GET(Get),
    SET(Set),
    PING(Ping),
    EXISTS(Exists),
    RPUSH,
    LPUSH,
    BLPOP,
    BRPOP,
}

impl Command {
    pub fn from_frame(frame: &Frame) -> Command {
        Command::GET(Get {
            key: "appy!".to_string(),
        })
    }
}

impl Select {
    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        handler.database = handler.databases.index(self.index as usize);
        let response = Frame::Simple("OK".to_string());
        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

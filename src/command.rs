use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use mini_redis::Result;
use std::sync::Arc;

pub mod ping;
pub use ping::Ping;

pub mod select;
pub use select::Select;

pub mod unknown;
pub use unknown::Unknown;

pub mod set;
pub use set::Set;

pub mod get;
pub use get::Get;

#[derive(Debug)]
struct Exists {}

#[derive(Debug)]
struct RPush {}

#[derive(Debug)]
struct LPush {}

#[derive(Debug)]
struct BLPop {}

#[derive(Debug)]
struct BRPop {}

#[derive(Debug)]
pub enum Command {
    SELECT(Select),
    GET(Get),
    SET(Set),
    PING(Ping),
    EXISTS(Exists),
    RPUSH(RPush),
    LPUSH(LPush),
    BLPOP(BLPop),
    BRPOP(BRPop),
    UNKNOWN(Unknown),
}

impl Command {
    pub fn from_frame(frame: Frame) -> Result<Command> {
        let mut parsed = Parse::new(frame)?;
        let command_frame = parsed.next_string()?.to_lowercase();

        let command = match &command_frame[..] {
            "select" => Command::SELECT(Select::parse_frames(&mut parsed)?),
            "ping" => Command::PING(Ping::parse_frame(&mut parsed)?),
            "set" => Command::SET(Set::parse_frame(&mut parsed)?),
            "get" => Command::GET(Get::parse_frame(&mut parsed)?),
            _ => {
                return Ok(Command::UNKNOWN(Unknown::new(command_frame)));
            }
        };

        parsed.finish()?;
        Ok(command)
    }

    pub async fn apply(self, database: Arc<Database>) -> Result<Frame> {
        use Command::*;

        dbg!(&self);

        let response = match self {
            PING(cmd) => cmd.apply().await,
            GET(cmd) => cmd.apply(database).await,
            SET(cmd) => cmd.apply(database).await,
            UNKNOWN(cmd) => cmd.apply().await,
            _ => Ok(Frame::Simple("OK".to_string())),
        };

        dbg!(&response);

        return response;
    }
}

use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use crate::Result;
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

pub mod exists;
pub use exists::Exists;

pub mod lpush;
pub use lpush::LPush;

pub mod rpush;
pub use rpush::RPush;

pub mod blpop;
pub use blpop::BLPop;

pub mod brpop;
pub use brpop::BRPop;

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
            "select" => Command::SELECT(Select::parse_frame(&mut parsed)?),
            "ping" => Command::PING(Ping::parse_frame(&mut parsed)?),
            "set" => Command::SET(Set::parse_frame(&mut parsed)?),
            "get" => Command::GET(Get::parse_frame(&mut parsed)?),
            "exists" => Command::EXISTS(Exists::parse_frame(&mut parsed)?),
            "rpush" => Command::RPUSH(RPush::parse_frame(&mut parsed)?),
            "lpush" => Command::LPUSH(LPush::parse_frame(&mut parsed)?),
            "brpop" => Command::BRPOP(BRPop::parse_frame(&mut parsed)?),
            "blpop" => Command::BLPOP(BLPop::parse_frame(&mut parsed)?),
            _ => {
                return Ok(Command::UNKNOWN(Unknown::new(command_frame)));
            }
        };

        parsed.finish()?;
        Ok(command)
    }

    pub async fn apply(self, database: Arc<Database>) -> Result<Frame> {
        use Command::*;

        let response = match self {
            PING(cmd) => cmd.apply().await,
            GET(cmd) => cmd.apply(database).await,
            SET(cmd) => cmd.apply(database).await,
            EXISTS(cmd) => cmd.apply(database).await,
            LPUSH(cmd) => cmd.apply(database).await,
            RPUSH(cmd) => cmd.apply(database).await,
            BLPOP(cmd) => cmd.apply(database).await,
            BRPOP(cmd) => cmd.apply(database).await,
            UNKNOWN(cmd) => cmd.apply().await,
            _ => Ok(Frame::Simple("OK".to_string())),
        };

        return response;
    }
}

use crate::frame::Frame;
use crate::handler::Handler;
use crate::parse::Parse;
use bytes::Bytes;
use mini_redis::Result;

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
            _ => {
                return Ok(Command::UNKNOWN(Unknown::new(command_frame)));
            }
        };

        parsed.finish()?;
        Ok(command)
    }

    pub async fn apply(self, handler: &mut Handler) -> Result<()> {
        use Command::*;

        match self {
            SELECT(cmd) => cmd.apply(handler).await,
            PING(cmd) => cmd.apply(handler).await,
            GET(cmd) => cmd.apply(handler).await,
            SET(cmd) => cmd.apply(handler).await,
            EXISTS(cmd) => Ok(()),
            RPUSH(cmd) => Ok(()),
            LPUSH(cmd) => Ok(()),
            BLPOP(cmd) => Ok(()),
            BRPOP(cmd) => Ok(()),
            UNKNOWN(cmd) => cmd.apply(handler).await,
        }
    }
}

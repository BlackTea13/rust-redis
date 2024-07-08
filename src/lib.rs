pub mod server;

pub mod command;
pub use command::Command;

pub mod connection;
pub use connection::Connection;

pub mod database;
pub use database::{Database, Databases};

pub mod frame;
pub use frame::Frame;

pub mod handler;
pub use handler::Handler;

pub mod parse;
pub use parse::Parse;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub const IP: &str = "127.0.0.1";
pub const PORT: &str = "11111";
pub const NUM_DB: usize = 16;

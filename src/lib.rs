pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub const IP: &str = "127.0.0.1";
pub const PORT: &str = "6379";
pub const NUM_DB: usize = 16;

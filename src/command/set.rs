use crate::frame::Frame;
use crate::handler::Handler;
use bytes::Bytes;
use mini_redis::Result;

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
}

impl Set {
    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        let _ = handler.database.lock().unwrap().set(&self.key, &self.value);
        let response = Frame::Simple("OK".to_string());
        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

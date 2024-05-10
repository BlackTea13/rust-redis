use crate::frame::Frame;
use crate::handler::Handler;
use mini_redis::Result;

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        let response = match handler.database.lock().unwrap().get(&self.key) {
            Some(val) => Frame::Bulk(val),
            None => Frame::Null,
        };

        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

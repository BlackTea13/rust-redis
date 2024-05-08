use crate::frame::Frame;
use crate::handler::Handler;
use mini_redis::Result;

#[derive(Debug)]
pub struct Unknown {
    command: String,
}

impl Unknown {
    pub fn new(command: impl ToString) -> Unknown {
        Unknown {
            command: command.to_string(),
        }
    }

    pub async fn apply(&self, handler: &mut Handler) -> Result<()> {
        let response = Frame::Error(format!("Unknown command '{}'", self.command));
        handler.connection.write_frame(&response).await?;
        Ok(())
    }
}

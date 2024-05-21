use crate::frame::Frame;
use crate::Result;

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

    pub async fn apply(&self) -> Result<Frame> {
        let response = Frame::Error(format!("Unknown command '{}'", self.command));
        Ok(response)
    }
}

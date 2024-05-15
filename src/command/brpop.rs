use crate::database::Database;
use crate::frame::Frame;
use crate::parse::Parse;
use goms_mini_project1::Result;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct BRPop {
    keys: Vec<String>,
    timeout: f64,
}

impl BRPop {
    pub fn parse_frame(parse: &mut Parse) -> Result<BRPop> {
        let mut maybe_keys = Vec::new();
        let key = parse.next_string()?;
        maybe_keys.push(key);
        loop {
            if let Ok(element) = parse.next_string() {
                maybe_keys.push(element);
            } else {
                let timeout_string = maybe_keys.pop().unwrap().parse::<f64>();
                if let Ok(timeout) = timeout_string {
                    return Ok(BRPop {
                        keys: maybe_keys,
                        timeout: timeout,
                    });
                } else {
                    return Err("Error, timeout needs to be a float".into());
                }
            }
        }
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let mut acc_timeout: f64 = 0.0;
        let mut sleep_time: f64 = 100.0;
        if self.timeout == 0.0 {
            sleep_time = 0.0;
        }
        loop {
            for key in self.keys.iter() {
                let result = database.rpop(&key)?;
                if let Some(val) = result {
                    return Ok(Frame::Array(vec![
                        Frame::Bulk(key.clone().into()),
                        Frame::Bulk(val),
                    ]));
                }
            }

            acc_timeout = acc_timeout + sleep_time;
            let _ = sleep(Duration::from_millis(sleep_time as u64)).await;
            if acc_timeout > self.timeout {
                return Ok(Frame::Null);
            }
        }
    }
}

use crate::command::{Command, LPush, RPush};
use crate::database::{Client, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use goms_mini_project1::Result;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct BLPop {
    keys: Vec<String>,
    timeout: f64,
}

impl BLPop {
    pub fn parse_frame(parse: &mut Parse) -> Result<BLPop> {
        let mut maybe_keys = Vec::new();
        let key = parse.next_string()?;
        maybe_keys.push(key);
        loop {
            if let Ok(element) = parse.next_string() {
                maybe_keys.push(element);
            } else {
                let timeout_string = maybe_keys.pop().unwrap().parse::<f64>();
                if let Ok(timeout) = timeout_string {
                    return Ok(BLPop {
                        keys: maybe_keys,
                        timeout: timeout,
                    });
                } else {
                    return Err("Error, timeout needs to be a float".into());
                }
            }
        }
    }

    fn index_of_first_exist(&self, database: Arc<Database>) -> Option<usize> {
        self.keys.iter().position(|k| database.exists(k))
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let is_exist = self.index_of_first_exist(database);

        if matches!(is_exist, None) {
            let (tx, rx) = oneshot::channel();
            let client = Client {
                sender: tx,
                receiver: rx,
            };
            database.clients.push_back(client);

            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::LPUSH(lpush) => return Ok(Frame::Bulk(lpush.elements.pop
                    Command::RPUSH(rpush) => todo!(),
                    _ => todo!(),
                }
            }


        }
    }
}

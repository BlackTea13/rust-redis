use crate::database::{Client, ClientState, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use goms_mini_project1::Result;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

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

    fn index_of_first_exist(&self, database: Arc<Database>) -> Option<usize> {
        self.keys.iter().position(|k| database.exists(k))
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let key_index = self.index_of_first_exist(database.clone());

        if matches!(key_index, None) {
            let (tx, mut rx) = mpsc::channel(1);
            let client = Client {
                client_state: ClientState::BRPOP,
                keys: VecDeque::from(self.keys.clone()),
                sender: tx,
            };

            database.clients.lock().unwrap().push_back(client.into());

            let timeout_duration = if self.timeout <= 0.0 {
                Duration::MAX
            } else {
                Duration::from_millis(self.timeout as u64)
            };

            let response = match timeout(timeout_duration, rx.recv()).await {
                Ok(element) => {
                    database.clients.lock().unwrap().pop_front();
                    return Ok(Frame::Array(vec![
                        Frame::Bulk(self.keys[key_index.unwrap()].clone().into()),
                        Frame::Bulk(element.unwrap()),
                    ]));
                }
                Err(_) => Ok(Frame::Null),
            };

            return response;
        } else {
            let key_index = key_index.unwrap();
            let key = &self.keys[key_index];
            let result = database.rpop(key)?;
            if let Some(val) = result {
                return Ok(Frame::Array(vec![
                    Frame::Bulk(key.clone().into()),
                    Frame::Bulk(val),
                ]));
            } else {
                return Err("Key exists but rpop failed".into());
            }
        }
    }
}

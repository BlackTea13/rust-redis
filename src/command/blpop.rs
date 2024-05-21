use crate::database::{Client, ClientState, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use crate::Result;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

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
                return if let Ok(timeout) = timeout_string {
                    Ok(BLPop {
                        keys: maybe_keys,
                        timeout,
                    })
                } else {
                    Err("Error, timeout needs to be a float".into())
                }
            }
        }
    }

    fn index_of_first_exist(&self, database: Arc<Database>) -> Option<usize> {
        self.keys.iter().position(|k| database.exists(k))
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let key_index = self.index_of_first_exist(database.clone());

        if key_index.is_none() {
            let (tx, mut rx) = mpsc::channel(1);
            let client = Client {
                client_state: ClientState::BLPOP,
                keys: VecDeque::from(self.keys.clone()),
                sender: tx,
            };
            
            {
                let mut client_write_lock = database.clients.write().unwrap();
                for key in self.keys.iter() {
                    client_write_lock
                        .entry(key.into())
                        .and_modify(|d| d.push_back(client.clone()))
                        .or_insert({
                            let mut deque = VecDeque::new();
                            deque.push_back(client.clone());
                            deque
                        });
                }
            }

            let timeout_duration = if self.timeout <= 0.0 {
                Duration::MAX
            } else {
                Duration::from_millis(self.timeout as u64)
            };

            match timeout(timeout_duration, rx.recv()).await {
                Ok(Some(element)) => {
                    let (key, value) = element;
                    Ok(Frame::Array(vec![
                        Frame::Bulk(key.into()),
                        Frame::Bulk(value),
                    ]))
                }
                Ok(None) => Err("Received `None` type from sender".into()),
                Err(_) => Ok(Frame::Null),
            }
        } else {
            let key_index = key_index.unwrap();
            let key = &self.keys[key_index];
            let result = database.lpop(key)?;
            if let Some(val) = result {
                Ok(Frame::Array(vec![
                    Frame::Bulk(key.clone().into()),
                    Frame::Bulk(val),
                ]))
            } else {
                Err("Key exists but lpop failed".into())
            }
        }
    }
}

use crate::database::{ClientState, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use crate::Result;
use bytes::Bytes;
use std::cmp::min;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug)]
pub struct LPush {
    key: String,
    elements: VecDeque<Bytes>,
}

impl LPush {
    pub fn parse_frame(parse: &mut Parse) -> Result<LPush> {
        let key = parse.next_string()?;
        let mut elements = VecDeque::new();

        if let Ok(element) = parse.next_bytes() {
            elements.push_back(element)
        } else {
            return Err("Error, wrong number of arguments".into());
        }

        loop {
            if let Ok(element) = parse.next_bytes() {
                elements.push_back(element)
            } else {
                return Ok(LPush { key, elements });
            }
        }
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let mut elements = self.elements.clone();

        if database.is_clients_empty_for_key(&self.key) {
            database.lpush(&self.key, &elements.clone().into_iter().collect())?;
            Ok(Frame::Integer(self.elements.len() as u64))
        } else {
            let work: Vec<(_, _)> = {
                let mut waiters = database.clients.write().unwrap();
                let waiters = waiters.get_mut(&self.key).unwrap();

                let end = min(waiters.len(), self.elements.len());
                 (0..end)
                    .map_while(|_| if !waiters.is_empty() {
                        let mut client = waiters.pop_front().unwrap();
                        while client.sender.is_closed() {
                            client = waiters.pop_front().unwrap();
                        }
                        let element = match client.client_state {
                            ClientState::BLPOP => elements.pop_front().unwrap(),
                            ClientState::BRPOP => elements.pop_back().unwrap(),
                        };
                        Some((client, element))
                    } else {None})
                    .collect()
            };

            for (client, elem) in work {
                let _ = client.sender.send((self.key.clone(), elem.clone())).await;
                drop(client.sender);
            }

            database.lpush(&self.key, &elements.clone().into_iter().collect())?;
            Ok(Frame::Integer(self.elements.len() as u64))
        }
    }
}

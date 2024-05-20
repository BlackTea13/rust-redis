use crate::database::{ClientState, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use goms_mini_project1::Result;
use std::cmp::min;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug)]
pub struct RPush {
    key: String,
    elements: VecDeque<Bytes>,
}

impl RPush {
    pub fn parse_frame(parse: &mut Parse) -> Result<RPush> {
        let key = parse.next_string()?;
        let mut elements = VecDeque::new();

        if let Ok(element) = parse.next_bytes() {
            elements.push_front(element)
        } else {
            return Err("Error, wrong number of arguments".into());
        }

        loop {
            if let Ok(element) = parse.next_bytes() {
                elements.push_front(element)
            } else {
                return Ok(RPush { key, elements });
            }
        }
    }

    pub async fn apply(&self, database: Arc<Database>) -> Result<Frame> {
        let mut elements = self.elements.clone();

        if database.is_clients_empty_for_key(&self.key) {
            let _ = database.rpush(&self.key, &elements.clone().into_iter().collect())?;
            return Ok(Frame::Integer(self.elements.len() as u64));
        } else {
            let work: Vec<(_, _)> = {
                let mut waiters = database.clients.write().unwrap();
                let waiters = waiters.get_mut(&self.key).unwrap();

                let end = min(waiters.len(), self.elements.len());
                let jobs = (0..end)
                    .map(|_| {
                        let client = waiters.pop_front().unwrap();
                        let element = match client.client_state {
                            ClientState::BLPOP => elements.pop_front().unwrap(),
                            ClientState::BRPOP => elements.pop_back().unwrap(),
                        };
                        (client, element)
                    })
                    .collect();
                jobs
            };

            for (client, elem) in work {
                let _ = client.sender.send((self.key.clone(), elem.clone())).await;
                drop(client.sender);
            }

            let _ = database.rpush(&self.key, &elements.clone().into_iter().collect())?;
            return Ok(Frame::Integer(self.elements.len() as u64));
        }
    }
}

use crate::database::{ClientState, Database};
use crate::frame::Frame;
use crate::parse::Parse;
use bytes::Bytes;
use goms_mini_project1::Result;
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

        if database.is_clients_empty() {
            let _ = database.lpush(&self.key, &elements.clone().into_iter().collect())?;
            return Ok(Frame::Integer(self.elements.len() as u64));
        } else {
            let work: Vec<(_, _)> = {
                let mut waiters = database.get_clients_waiting_for_key(&self.key);
                let end = min(waiters.len(), self.elements.len());
                let jobs = (0..end)
                    .map(|_| {
                        let client = waiters.pop_front().unwrap();
                        let element = match client.client_state {
                            ClientState::BLPOP => elements.pop_front().unwrap(),
                            ClientState::BRPOP => elements.pop_back().unwrap(),
                        };
                        (client.sender.clone(), element)
                    })
                    .collect();
                jobs
            };

            for (sender, elem) in work {
                let _ = sender.send(elem.clone()).await;
            }

            let _ = database.lpush(&self.key, &elements.clone().into_iter().collect())?;
            return Ok(Frame::Integer(self.elements.len() as u64));
        }
    }
}

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Database {
    map: HashMap<String, Bytes>,
}

#[derive(Debug)]
pub struct Databases {
    pub databases: Vec<Arc<Mutex<Database>>>,
    pub senders: Vec<mpsc::Sender<String>>,
    pub receivers: Vec<mpsc::Receiver<String>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Option<Bytes> {
        self.map.get(key).cloned()
    }

    pub fn set(&mut self, key: &String, value: &Bytes) -> Option<Bytes> {
        self.map.insert(key.clone(), value.clone())
    }
}

impl Databases {
    pub fn new() -> Databases {
        let mut senders: Vec<mpsc::Sender<String>> = Vec::new();
        let mut receivers: Vec<mpsc::Receiver<String>> = Vec::new();

        for _ in 0..16 {
            let (tx, rx) = mpsc::channel(64);
            senders.push(tx);
            receivers.push(rx);
        }

        Databases {
            databases: vec![Arc::new(Mutex::new(Database::new())); 16],
            senders: senders,
            receivers: receivers,
        }
    }

    pub fn index(&self, index: usize) -> Arc<Mutex<Database>> {
        Arc::clone(&self.databases[index])
    }
}

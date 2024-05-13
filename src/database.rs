use bytes::Bytes;
use mini_redis::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct State {
    state: HashMap<String, Bytes>,
}

impl State {
    pub fn new() -> State {
        return State {
            state: HashMap::new(),
        };
    }

    pub fn insert(&mut self, key: &String, value: &Bytes) -> Option<Bytes> {
        self.state.insert(key.clone(), value.clone())
    }

    pub fn get(&self, key: &String) -> Option<Bytes> {
        self.state.get(key).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub database: Arc<Mutex<State>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            database: Arc::new(Mutex::new(State::new())),
        }
    }

    pub fn get(&self, key: &String) -> Option<Bytes> {
        self.database.lock().unwrap().get(key)
    }

    pub fn insert(&self, key: &String, value: &Bytes) -> Result<()> {
        self.database.lock().unwrap().insert(key, value);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Databases {
    pub databases: Vec<Arc<Database>>,
}

impl Databases {
    pub fn new() -> Databases {
        Databases {
            databases: vec![Arc::new(Database::new()); 16],
        }
    }
}

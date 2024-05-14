use crate::NUM_DB;
use bytes::Bytes;
use mini_redis::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct List {
    list: Vec<Bytes>,
}

#[derive(Debug)]
struct Value {
    value: Bytes,
}

#[derive(Debug)]
enum EntryValue {
    List(List),
    Value(Value),
}

#[derive(Debug)]
pub struct State {
    state: HashMap<String, EntryValue>,
}

impl State {
    pub fn new() -> State {
        return State {
            state: HashMap::new(),
        };
    }

    pub fn set(&mut self, key: &String, value: &Bytes) -> Result<()> {
        let value = Value {
            value: value.clone(),
        };

        let _ = self.state.insert(key.clone(), EntryValue::Value(value));
        Ok(())
    }

    pub fn lpush(&mut self, key: &String, values: &[&Bytes]) -> Result<()> {
        if self.exists(key) {
            Err("Type error".into())
        }
    }

    pub fn get(&self, key: &String) -> Option<Bytes> {
        self.state.get(key).cloned()
    }

    pub fn exists(&self, key: &String) -> bool {
        self.state.contains_key(key)
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
        self.database.lock().unwrap().insert_bytes(key, value);
        Ok(())
    }

    pub fn exists(&self, key: &String) -> bool {
        self.database.lock().unwrap().exists(key)
    }
}

#[derive(Debug)]
pub struct Databases {
    pub databases: Vec<Arc<Database>>,
}

impl Databases {
    pub fn new() -> Databases {
        Databases {
            databases: (0..NUM_DB).map(|_| Arc::new(Database::new())).collect(),
        }
    }
}

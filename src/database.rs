use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Database {
    map: HashMap<String, Bytes>,
}

#[derive(Debug)]
pub struct Databases {
    pub databases: Vec<Arc<Mutex<Database>>>,
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
        Databases {
            databases: vec![Arc::new(Mutex::new(Database::new())); 16],
        }
    }

    pub fn index(&self, index: usize) -> Arc<Mutex<Database>> {
        Arc::clone(&self.databases[index])
    }
}

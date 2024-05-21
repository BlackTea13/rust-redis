use bytes::Bytes;
use crate::{Result, NUM_DB};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;

#[derive(Debug)]
struct List {
    list: VecDeque<Bytes>,
}

impl List {
    fn new() -> List {
        return List {
            list: VecDeque::new(),
        };
    }

    fn push_front(&mut self, value: Bytes) {
        self.list.push_front(value)
    }

    fn push_back(&mut self, value: Bytes) {
        self.list.push_back(value)
    }

    fn pop_front(&mut self) -> Option<Bytes> {
        self.list.pop_front()
    }

    fn pop_back(&mut self) -> Option<Bytes> {
        self.list.pop_back()
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    value: Bytes,
}

impl Value {
    pub fn get_value(&self) -> &Bytes {
        return &self.value;
    }
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

    pub fn get(&self, key: &String) -> Result<Option<Value>> {
        let result = match self.state.get(key) {
            Some(val) => val,
            None => return Ok(None),
        };

        match result {
            EntryValue::Value(val) => Ok(Some(val.clone())),
            EntryValue::List(_) => return Err("Type error".into()),
        }
    }

    pub fn set(&mut self, key: &String, value: &Bytes) -> Result<()> {
        let value = Value {
            value: value.clone(),
        };

        let _ = self.state.insert(key.clone(), EntryValue::Value(value));
        Ok(())
    }

    pub fn lpush(&mut self, key: &String, values: &Vec<Bytes>) -> Result<()> {
        let result = if let Some(value) = self.state.get_mut(key) {
            value
        } else {
            let mut list = List::new();
            values.iter().for_each(|v| list.list.push_front(v.clone()));

            let _ = self.state.insert(key.clone(), EntryValue::List(list));
            return Ok(());
        };

        match result {
            EntryValue::List(list) => values.iter().for_each(|v| list.push_front(v.clone())),
            EntryValue::Value(_) => return Err("Type Error".into()),
        };

        return Ok(());
    }

    pub fn rpush(&mut self, key: &String, values: &Vec<Bytes>) -> Result<()> {
        let result = match self.state.get_mut(key) {
            Some(value) => value,
            None => {
                let mut list = List::new();
                values.iter().for_each(|v| list.list.push_back(v.clone()));

                let _ = self.state.insert(key.clone(), EntryValue::List(list));
                return Ok(());
            }
        };

        match result {
            EntryValue::List(list) => values.into_iter().for_each(|v| list.push_back(v.clone())),
            EntryValue::Value(_) => return Err("Type Error".into()),
        }

        return Ok(());
    }

    pub fn lpop(&mut self, key: &String) -> Result<Option<Bytes>> {
        let value = match self.state.get_mut(key) {
            Some(v) => v,
            None => return Ok(None),
        };

        match value {
            EntryValue::Value(_) => return Err("Type Error".into()),
            EntryValue::List(list) => return Ok(list.pop_front()),
        }
    }

    pub fn rpop(&mut self, key: &String) -> Result<Option<Bytes>> {
        let value = match self.state.get_mut(key) {
            Some(v) => v,
            None => return Ok(None),
        };

        match value {
            EntryValue::Value(_) => return Err("Type Error".into()),
            EntryValue::List(ref mut list) => return Ok(list.pop_back()),
        }
    }

    pub fn exists(&self, key: &String) -> bool {
        let is_some = self.state.get(key).is_some();
        let not_empty = if let Some(val) = self.state.get(key) {
            match val {
                EntryValue::Value(_) => true,
                EntryValue::List(list) => !list.list.is_empty(),
            }
        } else {
            false
        };
        is_some && not_empty
    }
}

#[derive(Debug)]
pub struct Database {
    pub database: Arc<Mutex<State>>,
    pub clients: Arc<RwLock<HashMap<String, VecDeque<Client>>>>,
}

#[derive(Debug, Clone)]
pub enum ClientState {
    BLPOP,
    BRPOP,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_state: ClientState,
    pub keys: VecDeque<String>,
    pub sender: mpsc::Sender<(String, Bytes)>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            database: Arc::new(Mutex::new(State::new())),
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &String) -> Result<Option<Value>> {
        self.database.lock().unwrap().get(key)
    }

    pub fn set(&self, key: &String, value: &Bytes) -> Result<()> {
        let _ = self.database.lock().unwrap().set(key, value);
        Ok(())
    }

    pub fn lpop(&self, key: &String) -> Result<Option<Bytes>> {
        self.database.lock().unwrap().lpop(key)
    }

    pub fn rpop(&self, key: &String) -> Result<Option<Bytes>> {
        self.database.lock().unwrap().rpop(key)
    }

    pub fn lpush(&self, key: &String, values: &Vec<Bytes>) -> Result<()> {
        let _ = self.database.lock().unwrap().lpush(key, values);
        Ok(())
    }

    pub fn rpush(&self, key: &String, values: &Vec<Bytes>) -> Result<()> {
        let _ = self.database.lock().unwrap().rpush(key, values);
        Ok(())
    }

    pub fn exists(&self, key: &String) -> bool {
        self.database.lock().unwrap().exists(key)
    }

    pub fn is_clients_empty_for_key(&self, key: &String) -> bool {
        let read_lock = self.clients.read().unwrap();
        !read_lock.contains_key(key) || (read_lock.get(key).iter().len() <= 0)
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

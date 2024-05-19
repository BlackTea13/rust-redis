use crate::command::Command;
use bytes::Bytes;
use goms_mini_project1::{Result, NUM_DB};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

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
        self.state.contains_key(key)
    }
}

#[derive(Debug)]
pub struct Database {
    pub database: Arc<Mutex<State>>,
    pub clients: VecDeque<Client>,
}

#[derive(Debug)]
pub struct Client {
    pub sender: oneshot::Sender<Command>,
    pub receiver: oneshot::Receiver<Command>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            database: Arc::new(Mutex::new(State::new())),
            clients: VecDeque::new(),
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

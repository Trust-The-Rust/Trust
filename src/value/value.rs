use std::collections::HashMap;
use std::str;
use std::sync::{Arc, Mutex};

pub struct Value {
    pub map: Arc<Mutex<HashMap<String, String>>>,
}

impl Value {
    pub fn new() -> Value {
        let mut map = Arc::new(Mutex::new(HashMap::new()));
        Value { map }
    }
    pub fn parse_command() {}

    pub fn create(&self, key: String, value: String) -> Result<bool, String> {
        let mut clone = self.map.clone();
        let mut map = clone.lock().unwrap();

        if map.contains_key(&key) {
            Err(String::from("has_value"))
        } else if key.is_empty() {
            Err(String::from("empty_key"))
        } else if value.is_empty() {
            Err(String::from("empty_value"))
        } else {
            map.insert(key, value);
            Ok(true)
        }
    }

    pub fn read(&self, key: String) -> Result<String, String> {
        let clone = self.map.clone();
        let map = clone.lock().unwrap();
        let str = map.get(&key);
        if let Some(str) = str {
            Ok(String::from(str))
        } else {
            Err(String::from("empty_key"))
        }
    }

    pub fn update(&self, key: String, value: String) -> Result<bool, String> {
        let mut clone = self.map.clone();
        let mut map = clone.lock().unwrap();

        if !map.contains_key(&key) {
            Err(String::from("has_value"))
        } else if key.is_empty() {
            Err(String::from("empty_key"))
        } else if value.is_empty() {
            Err(String::from("empty_value"))
        } else {
            map.insert(key, value);
            Ok(true)
        }
    }

    pub fn delete(&self, key: String) -> Result<bool, String> {
        let mut clone = self.map.clone();
        let mut map = clone.lock().unwrap();

        if map.contains_key(&key) {
            Err(String::from("has_value"))
        } else if key.is_empty() {
            Err(String::from("empty_key"))
        } else {
            map.remove(&key);
            Ok(true)
        }
    }
}

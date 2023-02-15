use std::collections::HashMap;

pub struct Value {
    pub map: HashMap<String, String>,
}

impl Value {
    pub fn new() -> Value {
        Value {
            map: HashMap::new(),
        }
    }
}

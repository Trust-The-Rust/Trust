use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
use std::sync::{Arc, Mutex};

pub struct List {
    pub map: HashMap<String, Vec<String>>,
}

impl List {
    pub fn new() -> List {
        List {
            map: HashMap::new(),
        }
    }
}

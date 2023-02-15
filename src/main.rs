mod command;
mod list;
mod value;

use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
use std::sync::{Arc, Mutex};
fn main() {
    let shared_value = Arc::new(Mutex::new(value::Value::new()));
    let shared_list = Arc::new(Mutex::new(list::List::new()));

    let data = Arc::new(Mutex::new(Vec::new()));

    let listener = TcpListener::bind("127.0.0.1:2606").unwrap();

    loop {
        let (socket, _) = listener.accept().unwrap();
        let data = data.clone();

        std::thread::spawn(move || {
            let mut socket = socket;
            let mut buffer = [0; 1024];

            loop {
                let bytes_read = socket.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }

                let mut data = data.lock().unwrap();
                data.extend_from_slice(&buffer[..bytes_read]);
                socket.write_all(&buffer[0..bytes_read]).unwrap();

                match str::from_utf8(&buffer) {
                    Ok(v) => println!("Value: {}", v),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            }
        });
    }
}

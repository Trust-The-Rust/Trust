use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:2606".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let mut buf = vec![0; 1024];
                let n = socket.read(&mut buf).await.unwrap();

                if n == 0 {
                    return;
                }

                match str::from_utf8(&buf) {
                    Ok(v) => println!("{} - {}", n, v),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}

// Uncomment this block to pass the first stage
use std::{io::Write, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // Do I really need to change something?
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => stream.write_all(b"+PONG\r\n").unwrap(),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

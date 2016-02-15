use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    println!("handling client");
    let mut request: String = "".to_owned();
    let read = stream.read_to_string(&mut request);
    println!("{}", request);
}

// accept connections and process them, spawning a new thread each one
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed");
            }
        }
    }
}

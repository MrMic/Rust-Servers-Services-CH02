use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening on 127.0.0.1:3001");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
    }
}

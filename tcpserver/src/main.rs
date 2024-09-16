use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Listening on 127.0.0.1:3001");
    for stream in connection_listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established");
    }
}

use core::str;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut connection_stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    connection_stream
        .write("Hello from the client".as_bytes())
        .unwrap();
    let mut buffer = [0; 1024];
    connection_stream.read(&mut buffer).unwrap();
    println!(
        "Received response from the server: {}",
        str::from_utf8(&buffer).unwrap(),
    );
}

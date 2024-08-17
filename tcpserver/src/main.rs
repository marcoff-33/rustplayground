use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on http://127.0.0.1:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // enstablishing connection & storing it in a buffer
                let mut _connection_stream = stream;
                println!("Connection established");
                let mut buffer = [0; 1024];
                _connection_stream.read(&mut buffer).unwrap();

                // response string pre-converted to bytes
                let response_string = "Hello, this is the server, your message was: ".as_bytes();

                // echoing back the message with our response string;
                let response = [response_string, &buffer].concat();
                _connection_stream.write(&response).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

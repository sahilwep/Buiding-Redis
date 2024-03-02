use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main(){
    // Create a TCP server listening on 8080
    let listener = TcpListener::bind("localhost:8080").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connected!");
                handle_client(stream);
            }
            Err(e) => {
                eprint!("Failed: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return;
        }

        stream.write_all(&buf[0..bytes_read]).expect("Failed to write to client");
    }
}
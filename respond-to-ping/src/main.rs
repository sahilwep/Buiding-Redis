use std::net::TcpListener;

fn main() {
    let _listener = TcpListener::bind("localhost:8080").unwrap();
}
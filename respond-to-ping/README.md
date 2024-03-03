# Respond to ping:

## Resource:

[TCP Overview](https://app.codecrafters.io/concepts/tcp-overview)
[Rust TCP Server](https://app.codecrafters.io/concepts/rust-tcp-server)

## Rust TCP Server:

* The `std::net` module

* Rust's `std::net` module provides access to networking primitives.

* To write TCP servers in Rust, you'll need to be familiar with the following methods:

```rust
    TcpListener::bind
    TcpListener::incoming
    TcpStream::connect
    TcpStream::read
    TcpStream::write_all
```

* We'll start by looking at `TcpStream::connect` and `TcpListener::bind`.

* `TcpStream::connect` is used to initiate outbound connections.

* Example usage:

```rust
use std::net::TcpStream;

// Connects to a TCP server running on localhost:8080
let stream = TcpStream::connect("localhost:8080")?;
```

* `TcpListener::bind`is used to to create servers to accept inbound connections.

* Example usage:

```rust
use std::net::TcpListener;

// Creates a TCP server listening on port 8080
let listener = TcpListener::bind("localhost:8080")?;
```

## The `TcpListener::bind` method

This is the interface for `TcpListener::bind`:

```rust
pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<TcpListener>
```

* To create a TCP server you'd specify a string like "localhost:8080" as addr:

```rust
// Starts a TCP server listening on localhost:8080
let listener = TcpListener::bind("localhost:8080")?;
```

## The TcpListener struct

* TcpListener is the struct returned from `TcpListener::bind`.

* Here are some methods associated with the TcpListener struct:

```rust
impl TcpListener {
    // accept waits for and returns the next connection to the listener
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)>

    // incoming returns an iterator over the connections being received on this listener
    pub fn incoming(&self) -> Incoming<TcpStream>

    // local_addr returns the local socket address of the listener
    pub fn local_addr(&self) -> Result<SocketAddr>
}
```
* Once you've created a listener, you can use `TcpListener::incoming()` to get an iterator over the incoming connections.

* This method returns an iterator that yields connections as they are accepted, allowing you to handle each new connection in a loop.

```rust
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            // handle the connection
        }
        Err(e) => {
            eprintln!("Failed: {}", e);
        }
    }
}
```

## The TcpStream *struct*

* The iterator returned from TcpListener::incoming yields instances of TcpStream.

* Some important methods associated with the TcpStream struct are:
```rust
impl TcpStream {
    // read reads bytes from the stream
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize>

    // write writes bytes to the stream and returns the number of bytes written.
    // It's often easier to use write_all instead of this method.
    pub fn write(&mut self, buf: &[u8]) -> Result<usize>

    // write_all writes all the bytes in buf to the stream
    pub fn write_all(&mut self, buf: &[u8]) -> Result<()>
}
```

* You can use `TcpStream::read()` and `TcpStream::write_all()` to read from and write to a TCP connection.

* To read data from a connection, you'll need to pass in a mutable byte slice to `TcpStream::read`. The data received will be stored in this byte slice. `TcpStream::read` returns a `Result<usize>` indicating the number of bytes read:

```rust
let mut buf = [0; 1024];
let n = stream.read(&mut buf)?;
println!("received {} bytes", n);
println!("data: {:?}", &buf[..n]);
```

* To write data to a connection, you'll need to pass in a byte slice to `TcpStream::write_all`. It returns a `Result<()>` indicating whether the write was successful:

```rust
let buf = b"hello world";
stream.write_all(buf)?;
```

* Using `TcpStream::write_all` is often easier than using `TcpStream::write` since it automatically handles the case where the entire buffer isn't written to the connection in one go.

* Now that you're familiar with TcpListener and TcpStream, let's see how to put them all together to create a simple TCP server that echoes all input it receives:

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Creates a TCP server listening on localhost:8080
    let listener = TcpListener::bind("localhost:8080").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
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
```

* There are some limitations you should know about the above example:
  * It can only handle one connection at a time. If a second client connects, it'll be blocked until the first client disconnects.
  * It doesn't handle errors gracefully. For example, if a client disconnects abruptly, the server will panic.

* We won't be covering how to fix these limitations in this concept. If you're interested in learning more, check out the Rust Book.

## Summary

* You've now learnt about how to use functions in the std::net module to build a TCP server.

* A quick recap of the functions and structs we covered:

`TcpListener::bind` : Returns a TcpListener instance
`TcpListener::incoming` : returns an iterator that yields instances of TcpStream as new connections are accepted
`TcpStream::read` : Reads data from a connection
`TcpStream::write_all` : Writes data to a connection

## Task Solution:

[Redis Protocol](https://redis.io/docs/reference/protocol-spec/)

* Code:

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind");    // create a listener server that listens on localhost port 6379
    println!("[+] TcpListener start at 127.0.0.1:6379");
    // handling incoming multiple incoming client through loop, this will make them in queue, while only one client will connect to our server.
    for stream in listener.incoming() {
        match stream {  // using match as it return in "Result", to handel handel success & failure 
            Ok(stream) => {
                // println!("{:?}",stream); // print this to know about the TcpStream details.
                println!("\n[+] Client has connected to server!");  // simple connected message will pop on server side.
                handle_client(stream);  // calling our function to handel our client
                println!("[+] Respond Successfully!");
            }
            Err(e) => {
                eprint!("Server can't connect to client: {}", e);   // handling error when our server fails to connect with client.
            }
        }
    }
}


// Create a function to handle our client
fn handle_client(mut stream: TcpStream) {

    let mut buf = [0; 512]; // Create an array of size 512, with default value '0', to store the incoming message that are comes from client.   NOTE : we take 512 max size, because as redis default size(it can be change manually) is 512mb to handel response from user.
    let _message = b"Hello Client!!";    // Message by server to client . This is an array of string.
    // using loop to handel specific user until we respond them.
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");    // this is used to read message from client
        
        // println!("{bytes_read}");   // this is the number of bytes of string.

        // this states that if we have not received anything from the client then we returns.
        if bytes_read == 0 {
            return;
        }
        // println!("message from client : {:?}", buf);    // this is the ASCII value of an array buf, that stores the client mess.
        
        println!("[+] Responding to the client!");
        stream.write_all(&_message[0..14]).expect("Failed to write to client"); // this statement used to respond the Client.
        // here [0..14]     (14) is the size of our message.

    }
}
```

* Output:
![Terminal image](/respond-to-ping/assets/img.png)
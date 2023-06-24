use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    // Build the connection

    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    //Receive and send messages

    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}

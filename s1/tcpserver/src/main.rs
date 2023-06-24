use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {

    //Bind the IP address and port to the listener

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000 ...");

    //Keep listening to the connection

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        println!("Connection establishment");
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}

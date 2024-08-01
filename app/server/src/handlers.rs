use std::{io::Write, net::TcpStream};

pub fn new_connection(mut stream : TcpStream){
    println!("Handling connection: {}", stream.peer_addr().unwrap());
    if let Err(e) = stream.write_all(b"OK\r\n") {
        eprintln!("Failed to write to stream: {}", e);
    }
}
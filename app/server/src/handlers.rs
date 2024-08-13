use std::{io::{Read, Write}, net::TcpStream};

pub fn new_connection(mut stream : TcpStream){
    println!("Handling request: {}", stream.peer_addr().unwrap());
    if let Err(e) = stream.write_all(b"OK\r\n") {
        eprintln!("Failed to write to stream: {}", e);
    }
    manage_request(stream);
}

fn manage_request(mut stream : TcpStream){
    let mut request = String::new();

    loop {
        match stream.read_to_string(&mut request) {
            Ok(0) => {
                println!("[INFO] client has ungracefully closed the connection {}", 
                    stream.peer_addr().unwrap());
                return;
            }
            Ok(_bytes_read) => {
                // disconnect message
                // client sending message
                // request messages
                println!("[INFO] received message from user: {}", request);
            }
            Err(e) => {
                eprintln!("[WARN] Error reading message client: {}", e);
            }
        }
    }
}
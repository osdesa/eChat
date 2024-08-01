use std::{io::{Read, Write}, net::TcpStream};

pub fn new_connection(mut stream : TcpStream){
    println!("Handling connection: {}", stream.peer_addr().unwrap());

    if let Err(e) = stream.write_all(b"OK\r\n") {
        eprintln!("Failed to write to stream: {}", e);
    }

    manage_connection(stream);
}

fn manage_connection(mut stream : TcpStream){
    let mut buffer: [u8; 512] = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("[INFO] client has ungracefully closed the connection {}", 
                    stream.peer_addr().unwrap());
                return;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                // disconnect message
                // client sending message
                // request messages

                println!("[INFO] received message from user: {}\n test", message);
            }
            Err(e) => {
                eprintln!("[WARN] Error reading message client: {}", e);
            }
        }
        println!("NEW LOOP");
    }
}
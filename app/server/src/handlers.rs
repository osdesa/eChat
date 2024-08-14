use std::{io::{Read, Write}, net::TcpStream};

use shared::MsgInfo;

pub fn new_connection(mut stream : TcpStream){
    println!("Handling request: {}", stream.peer_addr().unwrap());
    if let Err(e) = stream.write_all(b"OK\r\n") {
        eprintln!("Failed to write to stream: {}", e);
    }
    manage_request(stream);
}

fn manage_request(mut stream : TcpStream){
    let mut buf: [u8; 128] = [0; 128];

    loop {
        let msg : MsgInfo = shared::read_data(&mut stream);
        match msg.length {
            0 => {
                println!("[INFO] client has ungracefully closed the connection {}", 
                    stream.peer_addr().unwrap());
                return;
            }
            _bytes_read => {
                // disconnect message
                // client sending message
                // request messages
                println!("[INFO] received message from user: {}", msg.msg);
            }
        }
    }
}
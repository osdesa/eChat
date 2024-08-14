use std::{net::TcpStream, str::FromStr};

use shared::{Events, MsgInfo};


pub fn new_connection(mut stream : TcpStream){
    println!("Handling request: {}", stream.peer_addr().unwrap());

    shared::write_data(&mut stream, "OK".to_owned());

    manage_request(stream);
}

fn manage_request(mut stream : TcpStream){
    loop {
        let msg : MsgInfo = shared::read_data(&mut stream);
        match msg.length {
            0 => {
                println!("[INFO] client has ungracefully closed the connection {}", 
                    stream.peer_addr().unwrap());
                return;
            }
            _bytes_read => {valid_request(msg.msg);}
        }
    }
}

fn valid_request(msg : String) {
    match Events::from_str(&msg).unwrap() {
        Events::OK => todo!(),
        Events::GetPubKey => todo!(),
        Events::PostPubKey => todo!(),
    }
}
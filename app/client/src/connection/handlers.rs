use std::{io::Write, net::TcpStream};

use super::socket::NetConnection;

pub fn send_message(connection : &mut NetConnection, msg : String) {
    println!("SENDING MESSAGE");
    if let Err(e) = connection.stream.write_all(b"OK\r\n") {
        eprintln!("Failed to write to stream: {}", e);
    }else{
        println!("SENT DATA");
    }
}

pub fn listen(stream : TcpStream) {

}
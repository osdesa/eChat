use std::{io::Write, net::TcpStream};

use super::socket::NetConnection;

pub fn send_message(connection : &mut NetConnection, msg : String) {
    shared::write_data(&mut connection.stream, msg);
}

pub fn listen(stream : TcpStream) {

}
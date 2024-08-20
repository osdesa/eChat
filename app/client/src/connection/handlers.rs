use std::{net::TcpStream, str::FromStr};
use rsa::RsaPublicKey;

use shared::Events;
use super::socket::NetConnection;

pub fn send_message_unencrypted(stream : &mut TcpStream, msg : String) {
    shared::write_data(stream, msg);
}

pub fn handle_message(connection : &mut NetConnection){
    let msg = listen(&mut connection.stream);

    match Events::from_str(&msg).unwrap() {
        Events::OK => {},
        Events::PostPubKey => send_public_key(connection),
        Events::GetPubKey => received_public_key(connection, msg),
    }
}

fn send_public_key(connection : &mut NetConnection){
    println!("[INFO] Sending public key");

    let key : String = shared::encode_pub_key(connection.public.clone());    
    let msg = format!("POSTPubKey {}", key);

    send_message_unencrypted(&mut connection.stream, msg);
}

fn received_public_key(connection : &mut NetConnection, msg : String){
    println!("[INFO] Received server pub key");

    let split : Vec<String> = msg.split(" ")
        .map(|x| x.to_string())
        .collect();

    let user = split[1].clone();
    let key: RsaPublicKey =  shared::decode_pub_key(split[1..].join(" "));

    connection.keys.insert(user, key);
}

fn listen(stream : &mut TcpStream) -> String {
    shared::read_data(stream).msg
}


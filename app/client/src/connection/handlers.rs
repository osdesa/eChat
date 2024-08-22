use std::{net::TcpStream, str::FromStr, sync::{Arc, Mutex, MutexGuard}};
use rsa::RsaPublicKey;

use shared::Events;
use crate::connection;

use super::socket::NetConnection;

pub fn send_message_unencrypted(stream : &mut TcpStream, msg : String) {
    shared::write_data(stream, msg);
}

pub fn handle_net_message(network : &mut NetConnection){
    let msg = listen(&mut network.stream);

    match Events::from_str(&msg).unwrap() {
        Events::OK => {},
        Events::PostPubKey => send_public_key(network),
        Events::GetPubKey => received_public_key(network, msg),
    }

    shared::write_data(&mut network.stream, "OK".to_string());
    handle_app_message(network);
}

fn send_public_key(mut network : &mut NetConnection){
    println!("[INFO] Sending public key");

    let key : String = shared::encode_pub_key(network.public.clone());    
    let msg = format!("POSTPubKey {}", key);

    send_message_unencrypted(&mut network.stream, msg);
}

fn received_public_key(mut network : &mut NetConnection, msg : String){
    println!("[INFO] Received server pub key");

    let split : Vec<String> = msg.split(" ")
        .map(|x| x.to_string())
        .collect();

    let user = split[1].clone();
    let key: RsaPublicKey =  shared::decode_pub_key(split[1..].join(" "));

    network.keys.insert(user, key);
}

fn listen(stream : &mut TcpStream) -> String {
    shared::read_data(stream).msg
}

pub fn key_exchange(connection: &mut NetConnection){    
    shared::write_data(&mut connection.stream, "GETPubKey".to_string());
}

pub fn handle_app_message(network : &mut NetConnection){
    if let Some(msg) = network.receiver.recv(){
        println!("MSG APP : {}", msg);
    }
}

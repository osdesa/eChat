use std::{net::TcpStream, sync::{Arc, Mutex}};
use rsa::{signature::digest::crypto_common::InnerUser, RsaPublicKey};

use crate::{handlers, state::server_state::ServerState};

pub fn ok(){
    
}

pub fn get_pub_key(stream : &mut TcpStream, state : Arc<Mutex<ServerState>>){
    let server_info = state.lock().unwrap();
    let msg = format!("GETPubKey SERVER {}", shared::encode_pub_key(server_info.public_key.clone()));
    
    handlers::send_data(stream, &state, &"YOU".to_string(),msg);
}

pub fn post_pub_key(state : Arc<Mutex<ServerState>>, msg : String){
    let split = shared::split_string(msg);

    let user = split[1].clone(); 
    let key: RsaPublicKey = shared::decode_pub_key(split[2..].join(" "));    
    
    println!("user key: {}", user);

    println!("INSERT key: {}", user);
    let mut network = state.lock().unwrap();
    network.user_keys.insert(user.clone(), key.clone());
}
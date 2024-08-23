use core::time;
use std::{fmt::format, io::{Read, Write}, net::TcpStream, os::linux::raw::stat, str::FromStr, sync::{Arc, Mutex}, thread, time::Duration};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use shared::{Events, MsgInfo};

use crate::{requests, state::server_state::ServerState};

pub fn new_connection(mut stream : TcpStream, state : Arc<Mutex<ServerState>>){
    println!("Handling request: {}", stream.peer_addr().unwrap());

    change_user_count(1, &state);
    show_user_count(&state);

    send_welcome(&mut stream, &state);

    manage_request(&mut stream, state);
}

fn manage_request(stream : &mut TcpStream, state : Arc<Mutex<ServerState>>){
    loop {
        let msg : MsgInfo = read_data(stream, &state);

        match msg.length {
            0 => {
                println!("[INFO] client has ungracefully closed the connection ");
                change_user_count(-1, &state);
                return;
            }
            _bytes_read => {valid_request(msg.msg, state.clone(), stream);}
        }

        send_data(stream, &state, &"USER".to_string(), "OK".to_string());
    }
}

fn send_welcome(stream : &mut TcpStream, state : &Arc<Mutex<ServerState>>){
    println!("[INFO] Sending welcome");
    let network = state.lock().unwrap();
    let key = shared::encode_pub_key(network.public_key.clone());
    drop(network);

    let msg = format!("GETPubKey SERVER {}", key);
    // send the intended user name
    let user_msg: String = format!("{:<32}", "YOU_KEY");
    stream.write(user_msg.as_bytes());
    
    // send the size of the message
    let size: [u8; 4] = (msg.len() as u32).to_be_bytes();
    stream.write(&size);

    // send the encrypted message
    stream.write(msg.as_bytes());
}

pub fn send_data(stream : &mut TcpStream, state : &Arc<Mutex<ServerState>>, user : &String, data : String){
    // get the public key of the user
    let network = state.lock().unwrap();
    let pub_key: RsaPublicKey = match network.user_keys.get(user) {
        Some(key) => key.clone(),
        None => get_key(stream, &state, user),
    };
    drop(network);

    // encrypt message
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes())
        .expect("failed to encrypt");

    // send the intended user name
    let user_msg = format!("{:<32}", user);
    stream.write(user_msg.as_bytes());
    
    // send the size of the message
    let size: [u8; 4] = (enc_data.len() as u32).to_be_bytes();
    stream.write(&size);

    // send the encrypted message
    stream.write(&enc_data);
}

// RECURSIVE FUNCTION NO CHECKS
// Add checks to prevent infinite recursion
fn get_key(stream : &mut TcpStream, state : &Arc<Mutex<ServerState>>, user : &String) -> RsaPublicKey{
    // Send request for the users key to server
    println!("[WARN] recursive loop for getkey");
    let msg = format!("GETKEY");

    manage_request(stream, state.clone());
    
    send_data(stream, &state, &"USER".to_string(), msg);

    //thread::sleep(Duration::from_millis(4000));

    // recusive call to get the key
    let network = state.lock().unwrap();
    let keys = network.user_keys.clone();
    drop(network);

    let pub_key: RsaPublicKey = match keys.get(user) {
        Some(key) => key.clone(),
        None => get_key(stream, state, user),
    };

    // Store the key and return out
    let mut network = state.lock().unwrap();
    println!("INSERT key: {}", user);
    network.user_keys.insert(user.clone(), pub_key.clone());
    pub_key
}


fn read_data(stream : &mut TcpStream, state : &Arc<Mutex<ServerState>>) -> MsgInfo{
    let mut user_bytes : [u8; 32] = [0u8; 32];
    let mut length_bytes: [u8; 4] = [0u8; 4];

    if let Err(_) = stream.read_exact(&mut user_bytes) {
        println!("Client disconnected.");
    }
    let user = String::from_utf8_lossy(&user_bytes).to_string();

    if let Err(_) = stream.read_exact(&mut length_bytes) {
        println!("Client disconnected.");
    }

    let length = u32::from_be_bytes(length_bytes) as usize;

    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer);
    let msg = String::from_utf8_lossy(&buffer).to_string();

    if !user.contains("SERVER") || user.contains("KEY") {
        MsgInfo { msg: msg, length: length, user: user }
    }else{
        // decrypt the message
        let network = state.lock().unwrap();
        let key = network.private_key.clone();

        let data = String::from_utf8_lossy(&key.decrypt(Pkcs1v15Encrypt, &buffer).unwrap()).to_string();
        
        MsgInfo { msg: data, length: length, user: user }
    }

}

fn change_user_count(increment : i64, state : &Arc<Mutex<ServerState>>){
    let mut server_info = state.lock().unwrap();
    server_info.user_count += increment;
}

fn show_user_count(state: &Mutex<ServerState>) {
    let server_info = state.lock().unwrap();
    println!("User count: {}", server_info.user_count);
}

fn valid_request(msg : String, state : Arc<Mutex<ServerState>>, stream : &mut TcpStream) {
    match Events::from_str(&msg).unwrap_or_default() {
        Events::OK => requests::ok(),
        Events::GetPubKey => requests::get_pub_key(stream, state),
        Events::PostPubKey => requests::post_pub_key(state, msg),
    }
}
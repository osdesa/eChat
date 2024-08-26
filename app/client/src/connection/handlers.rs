use std::{io::{Read, Write}, str::FromStr, thread, time::Duration};
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use shared::{Events, MsgInfo};
use super::socket::NetConnection;

fn send_data(network : &mut NetConnection, msg : String, user : String){
    // check if the user has the key, and request if not
    let pub_key: RsaPublicKey = match network.keys.get(&user) {
        Some(key) => key.clone(),
        None => get_key(network, &user),
    };
    // encrypt message
    let mut rng = rand::thread_rng();

    let enc_data: Vec<u8> = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
        .expect("failed to encrypt");

    // send the intended user name
    let user_msg = format!("{:<32}", user);
    network.stream.write(user_msg.as_bytes());
    
    // send the size of the message
    let size: [u8; 4] = (enc_data.len() as u32).to_be_bytes();
    network.stream.write(&size);

    // send the encrypted message
    network.stream.write(&enc_data);
}

// RECURSIVE FUNCTION NO CHECKS
// Add checks to prevent infinite recursion
fn get_key(network: &mut NetConnection, user: &String) -> RsaPublicKey{
    println!("[INFO] Requesting key for {}", user);
    // Send request for the users key to server
    let msg = format!("GETKEY");
    send_data(network, msg, "SERVER".to_string());
    
    thread::sleep(Duration::from_millis(10));
    
    handle_net_message(network);

    // recusive call to get the key
    let pub_key: RsaPublicKey = match network.keys.get(user) {
        Some(key) => key.clone(),
        None => get_key(network, user),
    };

    // Store the key and return out
    network.keys.insert(user.clone(), pub_key.clone());
    pub_key
}

pub fn handle_net_message(network : &mut NetConnection){
    let msg = listen(network);

    match Events::from_str(&msg).unwrap() {
        Events::OK => {},
        Events::PostPubKey => send_public_key(network),
        Events::GetPubKey => received_public_key(network, msg),
        Events::Login => login_check(msg),
    }

    send_data(network, "OK".to_string(), "SERVER".to_owned());
    handle_app_message(network);
}

fn login_check(msg: String) {
    println!("LOGIN ATTEMPT: {}", msg);
}

fn send_public_key(network : &mut NetConnection){
    println!("[INFO] Sending public key");

    let key : String = shared::encode_pub_key(network.public.clone());    
    let msg = format!("PPK USER {}", key);

    // send the intended user name | Should be proper name |
    let user_msg: String = format!("{:<32}", "USER_KEY");
    network.stream.write(user_msg.as_bytes());
    
    // send the size of the message
    let size: [u8; 4] = (msg.len() as u32).to_be_bytes();
    network.stream.write(&size);

    // send the encrypted message
    network.stream.write(msg.as_bytes());
}

fn received_public_key(mut network : &mut NetConnection, msg : String){
    println!("[INFO] Received pub key");

    let split : Vec<String> = msg.split(" ")
        .map(|x| x.to_string())
        .collect();

    let user = split[1].clone();
    let key: RsaPublicKey =  shared::decode_pub_key(split[2..].join(" "));

    network.keys.insert(user, key);
    send_public_key(network);
}

fn listen(connection: &mut NetConnection) -> String {
    read_data(connection).msg
}

fn read_data(connection: &mut NetConnection) -> MsgInfo {
    let mut user_bytes : [u8; 32] = [0u8; 32];
    let mut length_bytes: [u8; 4] = [0u8; 4];
    
    if let Err(_) = connection.stream.read_exact(&mut user_bytes) {
        println!("Client disconnected.");
    }
    let user = String::from_utf8_lossy(&user_bytes).to_string();
    
    if let Err(_) = connection.stream.read_exact(&mut length_bytes) {
        println!("Client disconnected.");
    }
    let length = u32::from_be_bytes(length_bytes) as usize;
    
    let mut buffer = vec![0u8; length];
    connection.stream.read_exact(&mut buffer);
    let msg = String::from_utf8_lossy(&buffer).to_string();


    if user.contains("KEY") {
        // welcome, from server no encryption
        println!("[INFO] Server sent welcome");
        return MsgInfo { msg: msg, length: length, user: user }
    }else{
        // decrypt the message
        let decrypt_key = connection.private.clone();

        let dec_data = decrypt_key.decrypt(Pkcs1v15Encrypt, &buffer)
            .expect("failed to decrypt test");
   
        MsgInfo { msg: String::from_utf8_lossy(&dec_data).to_string(), length: length, user: user }
    }

}

pub fn handle_app_message(network : &mut NetConnection){
    if let Some(msg) = network.receiver.recv(){
        let split = shared::split_string(msg);

        match split[0].as_str(){
            "LOGIN" => login(split, network),
            _ => {},
        }
    }
}

fn login(msg : Vec<String>, network : &mut NetConnection){
    assert_eq!(msg.len(), 3, "login in message does not have 3 values");

    let username = msg[1].as_str();
    let password = msg[2].as_str();

    // validate username and password

    let data = format!("Login {} {}", username, password);

    send_data(network, data, "SERVER".to_string());
}
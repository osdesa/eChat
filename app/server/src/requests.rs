use std::{net::TcpStream, sync::{Arc, Mutex}};
use argon2::{password_hash::{Salt, SaltString}, Argon2, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;
use rsa::RsaPublicKey;

use crate::{database::interface, handlers, state::server_state::ServerState};

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

pub fn login(state: Arc<Mutex<ServerState>>, stream: &mut TcpStream, msg: String) {
    let split = shared::split_string(msg);
    
    let username = split[1].clone();
    let password = split[2].clone();

    println!("User: {} is trying to login", username);

    // get the users salt

    // hash the users password 
    let hash = hash_password(password);
    // check the database
    let valid = interface::check_user(username, hash);

    // change the state of the server and update the users key name

    // send the user the result
    println!("User is : {}", valid);
    let data = format!("Login {}", valid);

    handlers::send_data(stream, &state, &"USER".to_string(), data)
}

fn hash_password(password : String) -> String{
    let argon2 = Argon2::default();
    let salt : SaltString = SaltString::from_b64("cXdlcnR5cXdlcnR5cXdlcnR5cXdlcnR5cXdlcnR5").unwrap();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    
    Argon2::default().verify_password(password.as_bytes(), &hash).expect("invalid password");

    hash.to_string()
}
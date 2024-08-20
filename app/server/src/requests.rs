use std::net::TcpStream;
use rsa::RsaPublicKey;

pub fn ok(){
    println!("Received OK from user");
}

pub fn get_pub_key(stream : &mut TcpStream, key : RsaPublicKey){
    let msg = format!("GETPubKey {}", shared::encode_pub_key(key));
    shared::write_data(stream, msg);
    shared::write_data(stream, "POSTPubKey".to_string());
}

pub fn post_pub_key(){
    println!("Receiving pub key");
}
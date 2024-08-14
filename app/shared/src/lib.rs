use std::{fs, io::{Read, Write}, net::TcpStream, os::unix::fs::PermissionsExt, path::Path};
use rsa::{RsaPublicKey, RsaPrivateKey};
use pem;

// CONSTANTS
pub const PORT: u32 = 3214;
pub const DATA_CHUNK_SIZE: usize = 512;

// STRUCTURES
pub struct MsgInfo {
    pub msg : String,
    pub length : usize,
}

pub struct Keys {
    pub private : RsaPrivateKey,
    pub public : RsaPublicKey,
}

// FUNCTIONS

pub fn write_data(stream : &mut TcpStream, msg : String) {
    // Calculate the size of the message
    let size: [u8; 4] = (msg.len() as u32).to_be_bytes();

    stream.write_all(&size);
    stream.write_all(msg.as_bytes());
}

pub fn read_data(stream : &mut TcpStream) -> MsgInfo {
    let mut length_bytes = [0u8; 4];

    if let Err(_) = stream.read_exact(&mut length_bytes) {
        println!("Client disconnected.");
    }
    let length = u32::from_be_bytes(length_bytes) as usize;

    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer);

    MsgInfo {msg : String::from_utf8_lossy(&buffer).to_string(), length : length}
}

pub fn get_keys() {

}

pub fn key_generation() {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let priv_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key: RsaPublicKey = RsaPublicKey::from(&priv_key);

    store_keys(priv_key, pub_key);
}

fn store_keys(private : RsaPrivateKey, public : RsaPublicKey) {
    let key_directory = Path::new("secure_keys");

    let r = fs::create_dir_all(&key_directory).is_err();
    println!("{}", r);

    fs::set_permissions(&key_directory, fs::Permissions::from_mode(0o700));

    let private_key_path = key_directory.join("private_key.pem");
    let public_key_path = key_directory.join("public_key.pem");


    let p = pkcs1::EncodeRsaPrivateKey::write_pkcs1_pem_file(&private, private_key_path, pkcs1::LineEnding::CRLF).is_err();
    let pu = pkcs1::EncodeRsaPublicKey::write_pkcs1_pem_file(&public, public_key_path, pkcs1::LineEnding::CRLF).is_err();

    println!("{} {}", p, pu);
}
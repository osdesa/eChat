use std::{fs, io::{Read, Write}, net::TcpStream, os::unix::fs::PermissionsExt, path::Path, str::FromStr};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

// CONSTANTS
pub const PORT: u32 = 3214;
pub const DATA_CHUNK_SIZE: usize = 512;

// STRUCTURES
pub struct MsgInfo {
    pub msg : String,
    pub length : usize,
    pub user : String
}

pub struct Keys {
    pub private : RsaPrivateKey,
    pub public : RsaPublicKey,
}

pub enum Events {
    OK,
    GetPubKey,
    PostPubKey,
    Login,
}

impl FromStr for Events {
    type Err = ();

    fn from_str(input: &str) -> Result<Events, Self::Err> {
        let split = split_string(input.to_string());

        match split[0].as_str() {
            "OK"                => Ok(Events::OK),
            "GETPubKey"         => Ok(Events::GetPubKey),
            "PPK"               => Ok(Events::PostPubKey),
            "Login"             => Ok(Events::Login),
            _                   => Err(()),
        }
    }
}

// FUNCTIONS

pub fn split_string(s : String) -> Vec<String> {
    s.split(" ").map(|x| x.to_string()).collect()
}

pub fn write_data(stream : &mut TcpStream, msg : String) {
    // Calculate the size of the message
    let data = format!("{msg}");
    let size: [u8; 4] = (data.len() as u32).to_be_bytes();

    stream.write_all(&size);
    stream.write_all(data.as_bytes());
}

pub fn read_data(stream : &mut TcpStream, priv_key : Option<RsaPrivateKey>) -> MsgInfo {
    let mut length_bytes = [0u8; 4];

    if let Err(_) = stream.read_exact(&mut length_bytes) {
        println!("Client disconnected.");
    }
    let length = u32::from_be_bytes(length_bytes) as usize;

    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer);
    let msg = String::from_utf8_lossy(&buffer).to_string();

    if priv_key.is_some() {
        // decrypt the message
        decrypt_message(msg, priv_key.unwrap());
    }

    MsgInfo {msg : String::from_utf8_lossy(&buffer).to_string(), length : length, user : "".to_string()}
}

fn decrypt_message(msg: String, priv_key : RsaPrivateKey) -> String {
    println!("MSG: {}", msg);
    todo!()
}

pub fn get_keys(path : String) -> Keys{   
    println!("[INFO] Checking if user has keys"); 
    let loc = format!("secure_keys/{}", path);

    if !Path::new(&loc).exists(){
        println!("[INFO] Generating keys");
        key_generation(&loc);
    }

    println!("[INFO] Retrieving keys : {}", loc);
    read_keys(&loc)
}

pub fn encode_pub_key(key : RsaPublicKey) -> String {
    pkcs1::EncodeRsaPublicKey::to_pkcs1_pem(&key, pkcs1::LineEnding::CRLF).unwrap()
}

pub fn decode_pub_key(key : String) -> RsaPublicKey {
    pkcs1::DecodeRsaPublicKey::from_pkcs1_pem(&key).unwrap()
}

fn read_keys(path : &String) -> Keys {
    let key_directory = Path::new(path);
    let private_key_path = key_directory.join("private_key.pem");
    let public_key_path = key_directory.join("public_key.pem");

    let private: Result<RsaPrivateKey, pkcs1::Error> = pkcs1::DecodeRsaPrivateKey::read_pkcs1_pem_file(&private_key_path);
    let public: Result<RsaPublicKey, pkcs1::Error> = pkcs1::DecodeRsaPublicKey::read_pkcs1_pem_file(&public_key_path);

    Keys {private : private.unwrap(), public : public.unwrap()}
}

pub fn key_generation(path : &String) {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let priv_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key: RsaPublicKey = RsaPublicKey::from(&priv_key);

    store_keys(priv_key, pub_key, &path);
}

fn store_keys(private : RsaPrivateKey, public : RsaPublicKey, path : &String) {
    let key_directory = Path::new(path);

    fs::create_dir_all(&key_directory).is_err();

    fs::set_permissions(&key_directory, fs::Permissions::from_mode(0o700));

    let private_key_path = key_directory.join("private_key.pem");
    let public_key_path = key_directory.join("public_key.pem");

    pkcs1::EncodeRsaPrivateKey::write_pkcs1_pem_file(&private, private_key_path, pkcs1::LineEnding::CRLF).is_err();
    pkcs1::EncodeRsaPublicKey::write_pkcs1_pem_file(&public, public_key_path, pkcs1::LineEnding::CRLF).is_err();
}

pub fn write_encrypted(data: String, stream : &mut TcpStream, pub_key : RsaPublicKey) {
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data.as_bytes()).expect("failed to encrypt");

    let encrypted: String = String::from_utf8_lossy(&enc_data).to_string();
    
    write_data(stream, encrypted);
}
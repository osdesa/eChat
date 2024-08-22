use std::collections::{hash_map, HashMap};

use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct ServerState{
    pub ip_addr : String,
    pub port : u32,
    pub private_key : RsaPrivateKey,
    pub public_key : RsaPublicKey,
    pub user_count : i64,
    pub running : bool,
    pub user_keys : HashMap<String, RsaPublicKey>,
}

impl ServerState{
    pub fn new() -> Self {
        let keys = shared::get_keys("server".to_owned());
        ServerState {
            ip_addr: "127.0.0.1".to_string(),
            port: shared::PORT,
            private_key: keys.private,
            public_key: keys.public,
            user_count : 0,
            running: false,
            user_keys : HashMap::new(),
        }
    }
}
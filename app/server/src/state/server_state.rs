use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct ServerState{
    pub ip_addr : String,
    pub port : u32,
    pub private_key : RsaPrivateKey,
    pub public_key : RsaPublicKey,
    pub running : bool,
}

impl ServerState{
    pub fn new() -> Self {
        let keys = shared::get_keys("server".to_owned());
        ServerState {
            ip_addr: "127.0.0.1".to_string(),
            port: shared::PORT,
            private_key: keys.private,
            public_key: keys.public,
            running: false,
        }
    }
}
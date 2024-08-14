use std::{cmp::Ordering, collections::HashMap, io::Read, net::TcpStream};

use rsa::{RsaPrivateKey, RsaPublicKey};
use shared::Keys;

pub struct NetConnection {
    pub stream : TcpStream,
    port : u32,
    ip_addr : String,
    pub public : RsaPublicKey,
    pub private : RsaPrivateKey,
    pub keys : HashMap<String, RsaPublicKey>,
}

impl NetConnection {
    pub fn default(ip_addr : String, port : u32, keys : Keys) -> Self{
        let s = Self::connect_to_server(ip_addr.clone(), port);

        NetConnection{
            stream : s,
            port : port,
            ip_addr : ip_addr,       
            public : keys.public,
            private : keys.private,
            keys : HashMap::new(),
        }
    }

    fn connect_to_server(ip_addr : String, port : u32) -> TcpStream {
        TcpStream::connect(format!("{}:{}", ip_addr, port)).unwrap()
    }
}
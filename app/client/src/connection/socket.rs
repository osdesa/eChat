use std::{collections::HashMap, net::TcpStream, thread};

use fltk::app::Receiver;
use rsa::{RsaPrivateKey, RsaPublicKey};
use shared::Keys;

use crate::connection::handlers;

pub struct NetConnection {
    pub stream : TcpStream,
    _port : u32,
    _ip_addr : String,
    pub public : RsaPublicKey,
    pub private : RsaPrivateKey,
    pub receiver : Receiver<String>,
    pub keys : HashMap<String, RsaPublicKey>,
}

impl NetConnection {
    pub fn default(ip_addr : String, port : u32, keys : Keys, recv : Receiver<String>) -> Self{
        let s = Self::connect_to_server(ip_addr.clone(), port);

        NetConnection{
            stream : s,
            _port : port,
            _ip_addr : ip_addr,       
            public : keys.public,
            private : keys.private,
            receiver : recv,
            keys : HashMap::new(),
        }
    }

    fn connect_to_server(ip_addr : String, port : u32) -> TcpStream {
        TcpStream::connect(format!("{}:{}", ip_addr, port)).unwrap_or_else(|error| {
            panic!("[FAIL] Failed to connect: to server[{}:{}] reason: {}", ip_addr, port, error.to_string());
        })
        
    }    
}

pub fn net_connect(nr: &Receiver<String>) {
    println!("[INFO] Starting network thread");
    let keys = shared::get_keys("client".to_owned());
    let mut network = NetConnection::default(
        "127.0.0.1".to_string(), shared::PORT, keys, nr.clone());


    // Spawn network thread
    thread::spawn(move || {
        loop {
            handlers::handle_net_message(&mut network);
        }
    });
}
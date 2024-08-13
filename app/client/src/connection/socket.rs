use std::{cmp::Ordering, io::Read, net::TcpStream};

pub struct NetConnection {
    pub stream : TcpStream,
    port : u32,
    ip_addr : String,
}

impl NetConnection {
    pub fn default(ip_addr : String, port : u32) -> Self{
        let s = Self::connect_to_server(ip_addr.clone(), port);
        
        NetConnection{
            stream : s,
            port : port,
            ip_addr : ip_addr,            
        }
    }

    fn connect_to_server(ip_addr : String, port : u32) -> TcpStream {
        TcpStream::connect(format!("{}:{}", ip_addr, port)).unwrap()
    }

    pub fn check_connection(&mut self){
        let mut buf: [u8; 128] = [0; 128];
        let result = self.stream.read(&mut buf);

        println!("MSG: {}", String::from_utf8_lossy(&buf));

        
        if String::from_utf8_lossy(&buf).eq("OK\r\n") {
            println!("[WARN] Failed to receive welcome message");
        } else{
            println!("[INFO] Received welcome message");
        }
    }
}
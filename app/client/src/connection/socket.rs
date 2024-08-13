use std::net::TcpStream;

pub struct NetConnection {
    stream : TcpStream,
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
}
use std::net::TcpListener;
use std::{io, thread};

use crate::handlers;

pub fn start_server(port : u32) -> std::io::Result<TcpListener>{
    println!("[INIT] Starting server on port: {}", port);   

    let address = format!("127.0.0.1:{port}");
    let stream = TcpListener::bind(address)?;

    Ok(stream)
}

pub fn listen(listener : TcpListener) -> io::Result<()> {
    println!("[INIT] listening for connections");   

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handlers::new_connection(stream);
                });
            }
            Err(e) => {eprintln!("Failed to accept connection: {}", e)}
        }
    }
    Ok(())
}
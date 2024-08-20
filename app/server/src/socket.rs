use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::{io, thread};

use crate::handlers;
use crate::state::server_state::ServerState;

pub fn start_server(port : u32) -> std::io::Result<TcpListener>{
    println!("[INIT] Starting server on port: {}", port);   

    let address = format!("127.0.0.1:{port}");
    let stream = TcpListener::bind(address)?;

    Ok(stream)
}

pub fn listen(listener : TcpListener, state : Arc<Mutex<ServerState>>) -> io::Result<()> {
    println!("[INIT] listening for connections");   

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let server_state = Arc::clone(&state);
                thread::spawn(move || {
                    handlers::new_connection(stream, server_state);
                });
            }
            Err(e) => {eprintln!("Failed to accept connection: {}", e)}
        }
    }
    Ok(())
}
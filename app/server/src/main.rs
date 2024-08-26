mod socket;
mod handlers;
mod state;
mod requests;
mod database;

use std::sync::{Arc, Mutex};

use database::interface;

use crate::state::server_state::ServerState;

fn main() {
    println!("[INIT] Starting server init");

    println!("[INIT] Creating database");

    interface::init_database();
    //interface::filler();

    // Generate or read keys
    let state: Arc<Mutex<ServerState>> = Arc::new(Mutex::new(ServerState::new()));
    
    let listener = match socket::start_server(shared::PORT) {
        Ok(socket) => {
            println!("[INFO] Server started");
            socket
        },
        Err(error) => panic!("[FATAL] Failed to start server: {error:?}"),
    };
    
    let _ = socket::listen(listener, state);
}
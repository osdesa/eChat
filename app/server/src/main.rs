mod socket;
mod handlers;
mod state;

use crate::state::server_state::ServerState;

fn main() {
    println!("[INIT] Starting server init");

    // Generate or read keys
    let state = ServerState::new();
    
    let listener = match socket::start_server(shared::PORT) {
        Ok(socket) => {
            println!("[INFO] Server started");
            socket
        },
        Err(error) => panic!("[FATAL] Failed to start server: {error:?}"),
    };
    
    let _ = socket::listen(listener);
}
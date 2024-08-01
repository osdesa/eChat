mod socket;
mod handlers;
fn main() {
    println!("[INIT] Starting server init");

    let listener = match socket::start_server(shared::PORT) {
        Ok(socket) => {
            println!("[INFO] Server started");
            socket
        },
        Err(error) => panic!("[FATAL] Failed to start server: {error:?}"),
    };
    
    let _ = socket::listen(listener);
}
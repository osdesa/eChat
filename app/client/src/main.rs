use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Define the server address and port
    let server_address = "127.0.0.1:3214";
    
    // Establish a connection to the server
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_address);

            // Send a request to the server
            let request_message = b"Hello, server!";
            stream.write_all(request_message)?;

            // Buffer to read the response
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer)?;

            // Print the server response
            println!("Received response from server: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            Err(e)
        }
    }
}

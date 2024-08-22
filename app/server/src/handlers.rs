use std::{net::TcpStream, str::FromStr, sync::{Arc, Mutex}};
use shared::{Events, MsgInfo};


use crate::{requests, state::server_state::ServerState};


pub fn new_connection(mut stream : TcpStream, state : Arc<Mutex<ServerState>>){
    println!("Handling request: {}", stream.peer_addr().unwrap());

    change_user_count(1, &state);
    show_user_count(&state);

    shared::write_data(&mut stream, "OK".to_owned());

    manage_request(stream, state);
}


fn manage_request(mut stream : TcpStream, state : Arc<Mutex<ServerState>>){
    loop {
        let msg : MsgInfo = shared::read_data(&mut stream);
        match msg.length {
            0 => {
                println!("[INFO] client has ungracefully closed the connection ");
                change_user_count(-1, &state);
                return;
            }
            _bytes_read => {valid_request(msg.msg, state.clone(), &mut stream);}
        }

        shared::write_data(&mut stream, "OK".to_string());
    }
}

fn change_user_count(increment : i64, state : &Arc<Mutex<ServerState>>){
    let mut server_info = state.lock().unwrap();
    server_info.user_count += increment;
}

fn show_user_count(state: &Mutex<ServerState>) {
    let server_info = state.lock().unwrap();
    println!("User count: {}", server_info.user_count);
}

fn valid_request(msg : String, state : Arc<Mutex<ServerState>>, stream : &mut TcpStream) {
    let server_info = state.lock().unwrap();

    match Events::from_str(&msg).unwrap_or_default() {
        Events::OK => requests::ok(),
        Events::GetPubKey => requests::get_pub_key(stream, server_info.public_key.clone()),
        Events::PostPubKey => requests::post_pub_key(),
    }
}
use crate::connection::handlers;
use crate::state::app_state::AppState;
use crate::ui::components;
use crate::ui::login_screen::LoginScreen;
use crate::connection::socket::NetConnection;

use fltk::app::{self, App, Receiver, Sender};
use fltk::prelude::*;
use fltk::window::DoubleWindow;
pub struct EChat{
    state : AppState,
    win : DoubleWindow,
    app : App,
    net_connection : NetConnection,
    sender : Sender<u64>,
    receiver : Receiver<u64>,
}

impl EChat{
    pub fn new() -> Self{
        let (s, r) = app::channel();
        let keys = shared::get_keys("client".to_owned());
        EChat{
            state : AppState::new(),
            win: components::create_window(),
            app: components::create_app(),
            net_connection : NetConnection::default("127.0.0.1".to_owned(), shared::PORT, keys),
            sender : s,
            receiver : r,
        }
    }

    pub fn run(&mut self){
        println!("[INFO] App is running");

        self.init_screen();
        self.key_exchange();
        self.update();
    }

    pub fn display_sate(&self){
        println!("{}", self.state);
    }

    fn key_exchange(&mut self){
        handlers::send_message_unencrypted(&mut self.net_connection.stream, "GETPubKey".to_owned());
        handlers::handle_message(&mut self.net_connection);
    }

    fn update(&mut self){
        self.win.end();
        self.win.show();

        
        while self.app.wait() {
            handlers::handle_message(&mut self.net_connection);
            // Handle sent messages
            if let Some(msg) = self.receiver.recv(){
                match msg {
                    0 => {println!("CLOSE PROGRAM")},
                    _ => ()
                }
            }
        }
    }

    fn init_screen(&mut self){
        let mut current_screen = LoginScreen::default(self.sender);
        current_screen.register_default_callback();
    }
}
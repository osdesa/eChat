
use crate::state::app_state::AppState;
use crate::ui::components;
use crate::ui::login_screen::LoginScreen;
use crate::connection::socket::{self, NetConnection};

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
        EChat{
            state : AppState::new(),
            win: components::create_window(),
            app: components::create_app(),
            net_connection : NetConnection::default("127.0.0.1".to_owned(), shared::PORT),
            sender : s,
            receiver : r,
        }
    }

    pub fn run(&mut self){
        println!("[INFO] App is running");

        self.init_screen();
        self.connect_to_server();
        self.update();
    }

    pub fn display_sate(&self){
        println!("{}", self.state);
    }

    fn connect_to_server(&self){
        println!("[INFO] Connecting to server");
    }

    fn update(&mut self){
        self.win.end();
        self.win.show();

        while self.app.wait() {
            println!("TEST");

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
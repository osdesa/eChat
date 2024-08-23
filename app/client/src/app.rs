use crate::connection::socket;
use crate::state::app_state::AppState;
use crate::ui::components;
use crate::ui::login_screen::LoginScreen;

use fltk::app::{self, App, Receiver, Sender};
use fltk::prelude::*;
use fltk::window::DoubleWindow;

pub struct EChat{
    state : AppState,
    win : DoubleWindow,
    app : App,
    sender : Sender<String>,
    receiver : Receiver<String>,
}

impl EChat{
    pub fn new() -> Self{
        let (s, r) = app::channel();

        // Spawn network thread with receiver
        socket::net_connect(&r);

        EChat{
            state : AppState::new(),
            win: components::create_window(),
            app: components::create_app(),
            sender : s,
            receiver : r,
        }
    }

    pub fn run(&mut self){
        println!("[INFO] App is running");

        self.init_screen();
        self.update();
    }

    pub fn display_sate(&self){
        println!("{}", self.state);
    }

    fn update(&mut self){
        self.win.end();
        self.win.show();
        
        while self.app.wait() {
        }
    }

    fn init_screen(&mut self){
        let mut current_screen = LoginScreen::default();
        current_screen.register_default_callback(self.sender.clone());
    }
}
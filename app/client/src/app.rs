
use crate::state::app_state::AppState;
use crate::ui::components;
use crate::ui::login_screen::LoginScreen;

use fltk::app::App;
use fltk::prelude::*;
use fltk::window::DoubleWindow;
pub struct EChat{
    state : AppState,
    win : DoubleWindow,
    app : App,
}

impl EChat{
    pub fn new() -> Self{
        EChat{
            state : AppState::new(),
            win: components::create_window(),
            app: components::create_app(),
        }
    }

    pub fn run(&mut self){
        println!("[INFO] App is running");
        self.display_sate();
        self.init_screen();
    }

    pub fn display_sate(&self){
        println!("{}", self.state);
    }

    fn init_screen(&mut self){
        let current_screen = LoginScreen::default();

        self.win.end();
        self.win.show();
        self.app.run().unwrap();
    }
}
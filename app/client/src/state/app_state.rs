use std::fmt::{self};

use super::user_state::UserState;

pub enum Screen {
    Home,
    Login,
    Message,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Screen::Home => {write!(f, "Home")},
            Screen::Login => {write!(f, "Login")},
            Screen::Message => {write!(f, "Message")},
        }
    }
}

pub struct AppState{
    pub user : UserState,
    pub current_screen : Screen,
    pub change_screen : bool,
    pub update : bool,
}

impl AppState{
    pub fn new() -> Self {
        AppState {
           user : UserState::new(),
           current_screen : Screen::Login,
           change_screen : false,
           update : true,
        }
    }
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Username :{}\nAuth: {}\nScreen: {}\nChange: {}\nupdate: {}", 
        self.user.username, self.user.auth, self.current_screen, 
            self.change_screen, self.update)
    }
}
use crate::state::{self, app_state::AppState};

pub struct EChat{
    state : AppState,
}

impl EChat{
    pub fn new() -> Self{
        EChat{
            state : AppState::new(),
        }
    }

    pub fn run(&self){
        println!("[INFO] App is running");
        self.display_sate();
    }

    pub fn display_sate(&self){
        println!("{}", self.state);
    }
}
use fltk::{button::Button, input::{self}, prelude::WidgetBase};

pub struct LoginScreen{
    username_input : input::Input,
    password_input : input::Input,
    submit : Button, 
}

impl LoginScreen {
    pub fn new() -> Self {
        LoginScreen{
            username_input : input::Input::new(20, 20, 160, 25, "username"),
            password_input : input::Input::new(20, 220, 160, 25, "password"),
            submit : Button::new(20, 400, 160, 50, "Submit"),
        }
    }
}
use fltk::{app::Sender, button::{self, Button}, enums, frame, input::{self}, prelude::{InputExt, WidgetBase, WidgetExt}};
use fltk_grid::Grid;

use crate::login;

use super::components;

pub struct LoginScreen{
    grid : Grid,
    username_input : input::Input,
    password_input : input::Input,
    submit : Button, 
}

impl LoginScreen {
    pub fn default() -> Self {
        let mut grid = Grid::default_fill();
        grid.set_layout(20, 5);

        let username_input = input::Input::default();
        let password_input = input::Input::default();
        let submit = button::Button::default().with_label("Submit");

        let mut g = Self {
            grid,
            username_input,
            password_input,
            submit,
        };
        g.fill();
        g
    }
    
    fn fill(&mut self) {
        let grid = &mut self.grid;
        grid.show_grid(false); // set to true to see cell outlines

        let mut title = frame::Frame::default().with_label("Employee Form");
        title.set_frame(enums::FrameType::FlatBox);
        title.set_color(enums::Color::Red);
        title.set_label_color(enums::Color::White);
        grid.set_widget(&mut title,0,1..4,).unwrap();

        grid.set_widget(&mut self.username_input, 2, 2).unwrap();
        
        grid.set_widget(&mut self.password_input, 5, 2).unwrap();
        
        components::create_button_grid(grid, &mut self.submit, 10, 2);
    }

    pub fn register_default_callback(&mut self, sender : Sender<String>) {
        self.submit.set_callback({
            let username = self.username_input.clone();
            let password = self.password_input.clone();

            move |_| {
                println!("SENDING MESSAGE");
                sender.send(format!("LOGIN {} {}", username.value(), password.value()));
            }
        });
    }
}
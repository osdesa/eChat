use fltk::{button::{self, Button}, enums, frame, input::{self}, prelude::{WidgetBase, WidgetExt}};
use fltk_grid::Grid;

pub struct LoginScreen{
    grid : Grid,
    username_input : input::Input,
    password_input : input::Input,
    submit : Button, 
}

impl LoginScreen {
    pub fn default() -> Self {
        let mut grid = Grid::default_fill();
        grid.set_layout(10, 5);
        println!("test");

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
        grid.set_widget(&mut title,0,1..4,);
        grid.set_widget(&mut frame::Frame::default().with_label("Name"), 2, 1);
        grid.set_widget(&mut self.username_input, 2, 3);
        grid.set_widget(&mut frame::Frame::default().with_label("Age"), 4, 1);
        grid.set_widget(&mut self.password_input, 4, 3);
        grid.set_widget(&mut self.submit, 8, 2);  
    }
}
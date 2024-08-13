use fltk::{app, button::Button, prelude::{GroupExt, WidgetExt}, window::Window};
use fltk_grid::Grid;


pub fn create_app() -> app::App {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    app::background(80, 80, 80);

    app
}

pub fn create_window() -> fltk::window::DoubleWindow {
    let mut wind = Window::default().with_label("EChat").with_size(500, 300);
    wind.make_resizable(true);

    wind
}

pub fn create_button_grid(grid : &mut Grid, btn : &mut Button, row : usize, col : usize) {
    grid.set_widget(btn, row, col).unwrap()
}
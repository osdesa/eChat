use fltk::{app, prelude::WidgetExt, window::Window};


pub fn create_app() -> app::App {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    app
}

pub fn create_window() -> fltk::window::DoubleWindow {
    let wind = Window::default().with_label("EChat");

    wind
}
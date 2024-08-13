use fltk::{app, prelude::{GroupExt, WidgetExt}, window::Window};


pub fn create_app() -> app::App {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    app
}

pub fn create_window() -> fltk::window::DoubleWindow {
    let mut wind = Window::default().with_label("EChat").with_size(500, 300);;
    wind.make_resizable(true);

    wind
}
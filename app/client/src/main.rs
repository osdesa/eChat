mod app;
mod state;
mod ui;
mod login;
mod connection;

use crate::app::EChat;

fn main() {
    let keys = shared::get_keys();

    let mut app = EChat::new();
    app.run();
}

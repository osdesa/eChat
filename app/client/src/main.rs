mod app;
mod state;
mod ui;
mod login;
mod connection;

use crate::app::EChat;

fn main() {
    shared::key_generation();

    let mut app = EChat::new();
    app.run();
}

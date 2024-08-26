mod app;
mod state;
mod ui;
mod connection;

use crate::app::EChat;

fn main() {
    let mut app = EChat::new();
    app.run();
}

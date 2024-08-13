mod app;
mod state;
mod ui;

use crate::app::EChat;

fn main() {
    let mut app = EChat::new();
    app.run();
}

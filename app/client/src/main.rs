mod app;
mod state;

use crate::app::EChat;

fn main() {
    let app = EChat::new();
    app.run();
}

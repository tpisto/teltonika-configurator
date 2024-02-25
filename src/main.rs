use gpui::*;

mod db;
mod hello;
mod paths;
mod theme;

use hello::HelloWorld;

fn main() {
    tracing_subscriber::fmt::init();

    App::new().run(|cx: &mut AppContext| {
        // Open all windows and start the program
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld::new())
        });
    });
}

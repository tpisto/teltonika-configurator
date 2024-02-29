use gpui::*;

mod assets;
mod component_tree;
mod db;
mod hello;
mod paths;
mod theme;

use assets::Assets;
use hello::HelloWorld;
use paths::Paths;

pub enum CounterEvent {
    Increase { amount: i32 },
    Decrease { amount: i32 },
}
impl EventEmitter<CounterEvent> for HelloWorld {}

fn main() {
    tracing_subscriber::fmt::init();

    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        // Displays
        let displays = cx.displays();

        let mut window_options = WindowOptions::default();
        window_options.display_id = Some(displays[6].id());

        cx.open_window(window_options, |cx| {
            // Root view
            HelloWorld::new(cx)
        });
    });
}

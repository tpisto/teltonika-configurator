use gpui::*;

mod assets;
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
        load_fonts(cx).expect("Failed to load fonts");

        // Displays
        let displays = cx.displays();

        let mut window_options = WindowOptions::default();
        window_options.display_id = Some(displays[1].id());

        cx.open_window(window_options, |cx| {
            // Root view
            HelloWorld::new(cx)
        });
    });
}

fn load_fonts(cx: &mut AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx.asset_source().load(&font_path)?;
            embedded_fonts.push(font_bytes);
        }
    }
    cx.text_system().add_fonts(embedded_fonts)
}

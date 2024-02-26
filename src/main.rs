use std::io::Read;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use gpui::*;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

mod db;
mod hello;
mod paths;
mod theme;

use hello::HelloWorld;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

fn main() {
    tracing_subscriber::fmt::init();

    App::new().run(|cx: &mut AppContext| {
        // First load file FMT100.gpuiml from "ui" directory directly to string
        let mut xml = String::new();
        std::fs::File::open("ui/FMT100.gpuiml")
            .unwrap()
            .read_to_string(&mut xml)
            .unwrap();

        let hello_view = HelloWorld::new(xml);
        let hello_root_component = hello_view.root_component.clone();

        // First we start the file watcher
        cx.spawn(|mut cx| async move {
            let (mut watcher, mut rx) = async_watcher().unwrap();

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher
                .watch(std::path::Path::new("ui"), RecursiveMode::Recursive)
                .unwrap();

            while let Some(res) = rx.next().await {
                match res {
                    Ok(event) => match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            println!("File changed: {:?}", event.attrs);
                            let mut xml = String::new();
                            std::fs::File::open("ui/FMT100.gpuiml")
                                .unwrap()
                                .read_to_string(&mut xml)
                                .unwrap();
                            let mut root_component = hello_root_component.lock().unwrap();
                            *root_component = crate::hello::parse_component(xml);

                            let _ = cx.refresh();
                        }
                        _ => {}
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        })
        .detach();

        // Open all windows and start the program
        cx.open_window(WindowOptions::default(), |cx| cx.new_view(|_cx| hello_view));
    });
}

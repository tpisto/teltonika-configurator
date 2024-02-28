use gpui::*;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs::File, sync::{Arc, Mutex}};
use std::{borrow::Cow, io::Read};

use crate::component_tree::*;

pub enum FileChangeEvent {
    DataChange,
}
impl EventEmitter<FileChangeEvent> for HelloWorld {}

pub struct HelloWorld {
    pub text: SharedString,
    pub root_component: crate::component_tree::Component,
}

impl HelloWorld {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let xml = HelloWorld::read_xml_file();
        let this = Self {
            text: "Hello, World!".into(),
            root_component: parse_component(xml),
        };

        let view = cx.new_view(|_cx| this);

        // Listen for file change events. Now file change are triggered on this view, but later
        // we can move the file listener to somewhere else
        cx.subscribe(
            &view,
            |subscriber, emitter: &FileChangeEvent, cx| match emitter {
                FileChangeEvent::DataChange => {
                    subscriber.update(cx, |this, cx| {
                        this.root_component = parse_component(HelloWorld::read_xml_file());
                        cx.notify();
                    });
                }
                _ => {}
            },
        )
        .detach();

        // First we start the file watcher
        let view_clone = view.clone();
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
                        EventKind::Modify(modify_kind) => match modify_kind {
                            notify::event::ModifyKind::Data(_) => {
                                cx.update_view(&view_clone, |this, cx| {
                                    cx.emit(FileChangeEvent::DataChange);
                                    cx.notify();
                                });
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        })
        .detach();

        view
    }

    pub fn read_xml_file() -> String {
        // First load file FMT100.gpuiml from "ui" directory directly to string
        let mut xml = String::new();
        std::fs::File::open("ui/FMT100.gpuiml")
            .unwrap()
            .read_to_string(&mut xml)
            .unwrap();

        xml
    }
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Pass a reference to the locked component to render_component
        render_component(&self.root_component)
    }
}

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

use std::borrow::Cow;

use gpui::*;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

pub struct HelloWorld {
    pub text: SharedString,
    pub root_component: Component,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            text: "Hello, World!".into(),
            root_component: parse_component(),
        }
    }
}

#[derive(Debug)]
pub struct Component {
    pub elem: String,
    pub text: Option<String>,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Component>,
}

fn parse_component() -> Component {
    let xml = r##"
        <div flex size_full>
            <div flex size_full bg="#0000ff">                
                <div flex size_full bg="#ff00ff" justify-center>Moikka</div>
            </div>
            <div flex size_full bg="#ff0000" />
            <div flex size_full bg="#00ff00">
                <div />
            </div>                
        </div>
    "##;

    let mut reader = Reader::from_str(xml);
    reader
        .expand_empty_elements(true)
        .check_end_names(true)
        .trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Component> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(event) => match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    let elem_name = String::from_utf8(e.local_name().as_ref().to_vec()).unwrap();
                    let attributes = e
                        .html_attributes()
                        .map(|a| {
                            if let Ok(a) = a {
                                (
                                    String::from_utf8(a.key.local_name().as_ref().to_vec())
                                        .unwrap(),
                                    a.decode_and_unescape_value(&reader).unwrap().into_owned(),
                                )
                            } else {
                                // println!("Attributes are: {:?}", e.attributes());
                                // panic!("Error reading attribute");
                                ("error".to_string(), "error".to_string())
                            }
                        })
                        .collect::<Vec<(String, String)>>();

                    let component = Component {
                        elem: elem_name,
                        text: None,
                        attributes,
                        children: Vec::new(),
                    };

                    if let Event::Empty(_) = event {
                        // For Event::Empty, add directly to the parent if exists
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(component);
                        }
                    } else {
                        // For Event::Start, push onto the stack for potential nesting
                        stack.push(component);
                    }
                }
                Event::End(_) => {
                    if stack.len() > 1 {
                        let finished_component = stack.pop().unwrap();
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(finished_component);
                        }
                    }
                }
                Event::Text(e) => {
                    let text = e.unescape().unwrap();
                    if let Some(parent) = stack.last_mut() {
                        parent.text = Some(text.into_owned());
                    }
                }
                _ => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    stack.pop().unwrap_or_else(|| Component {
        elem: "error".to_string(),
        text: Some("error".to_string()),
        attributes: vec![],
        children: vec![],
    })
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        println!("Rendering HelloWorld: {:#?}", self.root_component);

        // Go through the component tree and create the div elements
        render_component(&self.root_component)
    }
}

fn render_component(component: &Component) -> impl IntoElement {
    match component.elem.as_str() {
        "div" => {
            let mut element = div();
            // Apply attributes
            for (k, v) in &component.attributes {
                match k.as_str() {
                    "bg" => element = element.bg(hex_to_rgb(v)),
                    "flex" => element = element.flex(),
                    "size_full" => element = element.size_full(),
                    "justify-center" => element = element.justify_center(),
                    _ => { /* ignore unknown attributes */ }
                }
            }
            // Recursively render children and add them
            if !component.children.is_empty() {
                let children_elements = component.children.iter().map(render_component);
                element = element.children(children_elements);
            }

            // Add text if exists
            if let Some(text) = &component.text {
                element = element.child(text.clone());
            }

            element
        }
        // Handle other element types as needed
        _ => div(),
    }
}

// Convert #RRGGBB to rgb(0x000000) format where 0x000000 is the hex value of the color in integer
// rgb is function call to convert hex to rgb
fn hex_to_rgb(hex: &str) -> Rgba {
    let hex = hex.trim_start_matches('#');
    let r = u32::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u32::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u32::from_str_radix(&hex[4..6], 16).unwrap();
    // u32 is the hex value of the color
    let value: u32 = (r << 16) + (g << 8) + b;
    rgb(value)
}

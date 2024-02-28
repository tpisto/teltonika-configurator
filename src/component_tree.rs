use gpui::*;

use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::io::Read;

#[derive(Debug)]
pub struct Component {
    pub elem: String,
    pub text: Option<String>,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Component>,
}

pub fn parse_component(xml: String) -> Component {
    let mut reader = Reader::from_str(xml.as_str());
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

pub fn render_component(component: &Component) -> impl IntoElement {
    match component.elem.as_str() {
        "div" => generic_render(div(), &component),
        "svg" => generic_render(svg(), &component),
        _ => generic_render(div(), &component),
    }
}

fn generic_render<T: Element + Styled + ?ParentElement>(
    mut element: T,
    component: &Component,
) -> impl IntoElement {
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

// fn set_attributes(element: dyn Element, attributes: Vec<(String, String)>) -> impl IntoElement {
//     // Apply attributes
//     for (k, v) in &attributes {
//         match k.as_str() {
//             "bg" => element = element.bg(hex_to_rgb(v)),
//             "flex" => element = element.flex(),
//             "size-full" => element = element.size_full(),
//             "block" => element = element.block(),
//             "absolute" => element = element.absolute(),
//             "relative" => element = element.relative(),
//             "visible" => element = element.visible(),
//             "invisible" => element = element.invisible(),
//             "overflow-hidden" => element = element.overflow_hidden(),
//             "overflow-x-hidden" => element = element.overflow_x_hidden(),
//             "overflow-y-hidden" => element = element.overflow_y_hidden(),

//             // Cursor
//             "cursor-default" => element = element.cursor_default(),
//             "cursor-pointer" => element = element.cursor_pointer(),
//             "cursor-text" => element = element.cursor_text(),
//             "cursor-move" => element = element.cursor_move(),
//             "cursor-not-allowed" => element = element.cursor_not_allowed(),
//             "cursor-context-menu" => element = element.cursor_context_menu(),
//             "cursor-crosshair" => element = element.cursor_crosshair(),
//             "cursor-vertical-text" => element = element.cursor_vertical_text(),
//             "cursor-alias" => element = element.cursor_alias(),
//             "cursor-copy" => element = element.cursor_copy(),
//             "cursor-no-drop" => element = element.cursor_no_drop(),
//             "cursor-grab" => element = element.cursor_grab(),
//             "cursor-grabbing" => element = element.cursor_grabbing(),
//             "cursor-col-resize" => element = element.cursor_col_resize(),
//             "cursor-row-resize" => element = element.cursor_row_resize(),
//             "cursor-n-resize" => element = element.cursor_n_resize(),
//             "cursor-e-resize" => element = element.cursor_e_resize(),
//             "cursor-s-resize" => element = element.cursor_s_resize(),
//             "cursor-w-resize" => element = element.cursor_w_resize(),
//             "justify-center" => element = element.justify_center(),
//             "justify-between" => element = element.justify_between(),
//             "justify-around" => element = element.justify_around(),
//             "justify-start" => element = element.justify_start(),
//             "justify-end" => element = element.justify_end(),
//             "items-start" => element = element.items_start(),
//             "items-end" => element = element.items_end(),
//             "items-center" => element = element.items_center(),
//             "flex-col" => element = element.flex_col(),
//             "flex-row" => element = element.flex_row(),
//             "flex-col_reverse" => element = element.flex_col_reverse(),
//             "flex-row_reverse" => element = element.flex_row_reverse(),
//             "flex-1" => element = element.flex_1(),
//             "flex-auto" => element = element.flex_auto(),
//             "flex-initial" => element = element.flex_initial(),
//             "flex-none" => element = element.flex_none(),
//             "flex-grow" => element = element.flex_grow(),
//             "flex-shrink" => element = element.flex_shrink(),
//             "flex-shrink_0" => element = element.flex_shrink_0(),
//             "shadow-sm" => element = element.shadow_sm(),
//             "shadow-md" => element = element.shadow_md(),
//             "shadow-lg" => element = element.shadow_lg(),
//             "shadow-xl" => element = element.shadow_xl(),
//             "shadow-2xl" => element = element.shadow_2xl(),
//             // Text styles like "text_sm", "text_lg", etc., would require parsing the value to apply correctly.
//             "text-color" => element = element.text_color(hex_to_rgb(v)),
//             "font-weight" => {
//                 // Parse v for font weight and apply
//             }

//             // Height
//             "h-0" => element = element.h_0(),
//             "h-1" => element = element.h_1(),
//             "h-2" => element = element.h_2(),
//             "h-3" => element = element.h_3(),
//             "h-4" => element = element.h_4(),
//             "h-5" => element = element.h_5(),
//             "h-6" => element = element.h_6(),
//             "h-8" => element = element.h_8(),
//             "h-10" => element = element.h_10(),
//             "h-12" => element = element.h_12(),
//             "h-16" => element = element.h_16(),
//             "h-20" => element = element.h_20(),
//             "h-24" => element = element.h_24(),
//             "h-32" => element = element.h_32(),
//             "h-40" => element = element.h_40(),
//             "h-48" => element = element.h_48(),
//             "h-56" => element = element.h_56(),
//             "h-64" => element = element.h_64(),
//             "h-72" => element = element.h_72(),
//             "h-80" => element = element.h_80(),
//             "h-96" => element = element.h_96(),
//             "h-auto" => element = element.h_auto(),
//             "h-full" => element = element.h_full(),
//             "h-1/2" => element = element.h_1_2(),
//             "h-1/3" => element = element.h_1_3(),
//             "h-2/3" => element = element.h_2_3(),
//             "h-1/4" => element = element.h_1_4(),
//             "h-2/4" => element = element.h_2_4(),
//             "h-3/4" => element = element.h_3_4(),
//             "h-1/5" => element = element.h_1_5(),
//             "h-2/5" => element = element.h_2_5(),
//             "h-3/5" => element = element.h_3_5(),
//             "h-4/5" => element = element.h_4_5(),
//             "h-1/6" => element = element.h_1_6(),
//             "h-5/6" => element = element.h_5_6(),
//             "h-1/12" => element = element.h_1_12(),

//             // Width
//             "w-0" => element = element.w_0(),
//             "w-1" => element = element.w_1(),
//             "w-2" => element = element.w_2(),
//             "w-3" => element = element.w_3(),
//             "w-4" => element = element.w_4(),
//             "w-5" => element = element.w_5(),
//             "w-6" => element = element.w_6(),
//             "w-8" => element = element.w_8(),
//             "w-10" => element = element.w_10(),
//             "w-12" => element = element.w_12(),
//             "w-16" => element = element.w_16(),
//             "w-20" => element = element.w_20(),
//             "w-24" => element = element.w_24(),
//             "w-32" => element = element.w_32(),
//             "w-40" => element = element.w_40(),
//             "w-48" => element = element.w_48(),
//             "w-56" => element = element.w_56(),
//             "w-64" => element = element.w_64(),
//             "w-72" => element = element.w_72(),
//             "w-80" => element = element.w_80(),
//             "w-96" => element = element.w_96(),
//             "w-auto" => element = element.w_auto(),
//             "w-full" => element = element.w_full(),
//             "w-1/2" => element = element.w_1_2(),
//             "w-1/3" => element = element.w_1_3(),
//             "w-2/3" => element = element.w_2_3(),
//             "w-1/4" => element = element.w_1_4(),
//             "w-2/4" => element = element.w_2_4(),
//             "w-3/4" => element = element.w_3_4(),
//             "w-1/5" => element = element.w_1_5(),
//             "w-2/5" => element = element.w_2_5(),
//             "w-3/5" => element = element.w_3_5(),
//             "w-4/5" => element = element.w_4_5(),
//             "w-1/6" => element = element.w_1_6(),
//             "w-5/6" => element = element.w_5_6(),
//             "w-1/12" => element = element.w_1_12(),
//             "min-h-0" => element = element.min_h_0(),
//             "min-h-full" => element = element.min_h_full(),
//             "min-w-0" => element = element.min_w_0(),
//             "min-w-full" => element = element.min_w_full(),
//             "max-h-0" => element = element.max_h_0(),
//             "max-h-full" => element = element.max_h_full(),
//             "max-w-0" => element = element.max_w_0(),
//             "max-w-full" => element = element.max_w_full(),

//             // Padding
//             "p-0" => element = element.p_0(),
//             "p-1" => element = element.p_1(),
//             "p-2" => element = element.p_2(),
//             "p-3" => element = element.p_3(),
//             "p-4" => element = element.p_4(),
//             "p-5" => element = element.p_5(),
//             "p-6" => element = element.p_6(),
//             "p-8" => element = element.p_8(),
//             "p-10" => element = element.p_10(),
//             "p-12" => element = element.p_12(),
//             "p-16" => element = element.p_16(),
//             "p-20" => element = element.p_20(),
//             "p-24" => element = element.p_24(),
//             "p-32" => element = element.p_32(),
//             "p-40" => element = element.p_40(),
//             "p-48" => element = element.p_48(),
//             "p-56" => element = element.p_56(),
//             "p-64" => element = element.p_64(),
//             "p-72" => element = element.p_72(),
//             "p-80" => element = element.p_80(),
//             "p-96" => element = element.p_96(),
//             "p-px" => element = element.p_px(),
//             "p-1/2" => element = element.p_1_2(),
//             "p-1/3" => element = element.p_1_3(),
//             "p-2/3" => element = element.p_2_3(),
//             "p-1/4" => element = element.p_1_4(),
//             "p-2/4" => element = element.p_2_4(),
//             "p-3/4" => element = element.p_3_4(),
//             "p-1/5" => element = element.p_1_5(),
//             "p-2/5" => element = element.p_2_5(),
//             "p-3/5" => element = element.p_3_5(),
//             "p-4/5" => element = element.p_4_5(),
//             "p-1/6" => element = element.p_1_6(),
//             "p-5/6" => element = element.p_5_6(),
//             "p-1/12" => element = element.p_1_12(),
//             "px-0" => element = element.px_0(),
//             "px-1" => element = element.px_1(),
//             "px-2" => element = element.px_2(),
//             "px-3" => element = element.px_3(),
//             "px-4" => element = element.px_4(),
//             "px-5" => element = element.px_5(),
//             "px-6" => element = element.px_6(),
//             "px-8" => element = element.px_8(),
//             "px-10" => element = element.px_10(),
//             "px-12" => element = element.px_12(),
//             "px-16" => element = element.px_16(),
//             "px-20" => element = element.px_20(),
//             "px-24" => element = element.px_24(),
//             "px-32" => element = element.px_32(),
//             "px-40" => element = element.px_40(),
//             "px-48" => element = element.px_48(),
//             "px-56" => element = element.px_56(),
//             "px-64" => element = element.px_64(),
//             "px-72" => element = element.px_72(),
//             "px-80" => element = element.px_80(),
//             "px-96" => element = element.px_96(),
//             "px-px" => element = element.px_px(),
//             "px-1/2" => element = element.px_1_2(),
//             "px-1/3" => element = element.px_1_3(),
//             "px-2/3" => element = element.px_2_3(),
//             "px-1/4" => element = element.px_1_4(),
//             "px-2/4" => element = element.px_2_4(),
//             "px-3/4" => element = element.px_3_4(),
//             "px-1/5" => element = element.px_1_5(),
//             "px-2/5" => element = element.px_2_5(),
//             "px-3/5" => element = element.px_3_5(),
//             "px-4/5" => element = element.px_4_5(),
//             "px-1/6" => element = element.px_1_6(),
//             "px-5/6" => element = element.px_5_6(),
//             "px-1/12" => element = element.px_1_12(),
//             "py-0" => element = element.py_0(),
//             "py-1" => element = element.py_1(),
//             "py-2" => element = element.py_2(),
//             "py-3" => element = element.py_3(),
//             "py-4" => element = element.py_4(),
//             "py-5" => element = element.py_5(),
//             "py-6" => element = element.py_6(),
//             "py-8" => element = element.py_8(),
//             "py-10" => element = element.py_10(),
//             "py-12" => element = element.py_12(),
//             "py-16" => element = element.py_16(),
//             "py-20" => element = element.py_20(),
//             "py-24" => element = element.py_24(),
//             "py-32" => element = element.py_32(),
//             "py-40" => element = element.py_40(),
//             "py-48" => element = element.py_48(),
//             "py-56" => element = element.py_56(),
//             "py-64" => element = element.py_64(),
//             "py-72" => element = element.py_72(),
//             "py-80" => element = element.py_80(),
//             "py-96" => element = element.py_96(),
//             "py-px" => element = element.py_px(),
//             "py-1/2" => element = element.py_1_2(),
//             "py-1/3" => element = element.py_1_3(),
//             "py-2/3" => element = element.py_2_3(),
//             "py-1/4" => element = element.py_1_4(),
//             "py-2/4" => element = element.py_2_4(),
//             "py-3/4" => element = element.py_3_4(),
//             "py-1/5" => element = element.py_1_5(),
//             "py-2/5" => element = element.py_2_5(),
//             "py-3/5" => element = element.py_3_5(),
//             "py-4/5" => element = element.py_4_5(),
//             "py-1/6" => element = element.py_1_6(),
//             "py-5/6" => element = element.py_5_6(),
//             "py-1/12" => element = element.py_1_12(),

//             // Margin
//             "m-0" => element = element.m_0(),
//             "m-1" => element = element.m_1(),
//             "m-2" => element = element.m_2(),
//             "m-3" => element = element.m_3(),
//             "m-4" => element = element.m_4(),
//             "m-5" => element = element.m_5(),
//             "m-6" => element = element.m_6(),
//             "m-8" => element = element.m_8(),
//             "m-10" => element = element.m_10(),
//             "m-12" => element = element.m_12(),
//             "m-16" => element = element.m_16(),
//             "m-20" => element = element.m_20(),
//             "m-24" => element = element.m_24(),
//             "m-32" => element = element.m_32(),
//             "m-40" => element = element.m_40(),
//             "m-48" => element = element.m_48(),
//             "m-56" => element = element.m_56(),
//             "m-64" => element = element.m_64(),
//             "m-72" => element = element.m_72(),
//             "m-80" => element = element.m_80(),
//             "m-96" => element = element.m_96(),
//             "m-px" => element = element.m_px(),
//             "m-1/2" => element = element.m_1_2(),
//             "m-1/3" => element = element.m_1_3(),
//             "m-2/3" => element = element.m_2_3(),
//             "m-1/4" => element = element.m_1_4(),
//             "m-2/4" => element = element.m_2_4(),
//             "m-3/4" => element = element.m_3_4(),
//             "m-1/5" => element = element.m_1_5(),
//             "m-2/5" => element = element.m_2_5(),
//             "m-3/5" => element = element.m_3_5(),
//             "m-4/5" => element = element.m_4_5(),
//             "m-1/6" => element = element.m_1_6(),
//             "m-5/6" => element = element.m_5_6(),
//             "m-1/12" => element = element.m_1_12(),
//             "mx-0" => element = element.mx_0(),
//             "mx-1" => element = element.mx_1(),
//             "mx-2" => element = element.mx_2(),
//             "mx-3" => element = element.mx_3(),
//             "mx-4" => element = element.mx_4(),
//             "mx-5" => element = element.mx_5(),
//             "mx-6" => element = element.mx_6(),
//             "mx-8" => element = element.mx_8(),
//             "mx-10" => element = element.mx_10(),
//             "mx-12" => element = element.mx_12(),
//             "mx-16" => element = element.mx_16(),
//             "mx-20" => element = element.mx_20(),
//             "mx-24" => element = element.mx_24(),
//             "mx-32" => element = element.mx_32(),
//             "mx-40" => element = element.mx_40(),
//             "mx-48" => element = element.mx_48(),
//             "mx-56" => element = element.mx_56(),
//             "mx-64" => element = element.mx_64(),
//             "mx-72" => element = element.mx_72(),
//             "mx-80" => element = element.mx_80(),
//             "mx-96" => element = element.mx_96(),
//             "mx-px" => element = element.mx_px(),
//             "mx-1/2" => element = element.mx_1_2(),
//             "mx-1/3" => element = element.mx_1_3(),
//             "mx-2/3" => element = element.mx_2_3(),
//             "mx-1/4" => element = element.mx_1_4(),
//             "mx-2/4" => element = element.mx_2_4(),
//             "mx-3/4" => element = element.mx_3_4(),
//             "mx-1/5" => element = element.mx_1_5(),
//             "mx-2/5" => element = element.mx_2_5(),
//             "mx-3/5" => element = element.mx_3_5(),
//             "mx-4/5" => element = element.mx_4_5(),
//             "mx-1/6" => element = element.mx_1_6(),
//             "mx-5/6" => element = element.mx_5_6(),
//             "mx-1/12" => element = element.mx_1_12(),
//             "my-0" => element = element.my_0(),
//             "my-1" => element = element.my_1(),
//             "my-2" => element = element.my_2(),
//             "my-3" => element = element.my_3(),
//             "my-4" => element = element.my_4(),
//             "my-5" => element = element.my_5(),
//             "my-6" => element = element.my_6(),
//             "my-8" => element = element.my_8(),
//             "my-10" => element = element.my_10(),
//             "my-12" => element = element.my_12(),
//             "my-16" => element = element.my_16(),
//             "my-20" => element = element.my_20(),
//             "my-24" => element = element.my_24(),
//             "my-32" => element = element.my_32(),
//             "my-40" => element = element.my_40(),
//             "my-48" => element = element.my_48(),
//             "my-56" => element = element.my_56(),
//             "my-64" => element = element.my_64(),
//             "my-72" => element = element.my_72(),
//             "my-80" => element = element.my_80(),
//             "my-96" => element = element.my_96(),
//             "my-px" => element = element.my_px(),
//             "my-1/2" => element = element.my_1_2(),
//             "my-1/3" => element = element.my_1_3(),
//             "my-2/3" => element = element.my_2_3(),
//             "my-1/4" => element = element.my_1_4(),
//             "my-2/4" => element = element.my_2_4(),
//             "my-3/4" => element = element.my_3_4(),
//             "my-1/5" => element = element.my_1_5(),
//             "my-2/5" => element = element.my_2_5(),
//             "my-3/5" => element = element.my_3_5(),
//             "my-4/5" => element = element.my_4_5(),
//             "my-1/6" => element = element.my_1_6(),
//             "my-5/6" => element = element.my_5_6(),
//             "my-1/12" => element = element.my_1_12(),
//             "m-auto" => element = element.m_auto(),
//             "m-full" => element = element.m_full(),
//             "mt-0" => element = element.mt_0(),
//             "mt-1" => element = element.mt_1(),
//             "mt-2" => element = element.mt_2(),
//             "mt-3" => element = element.mt_3(),
//             "mt-4" => element = element.mt_4(),
//             "mt-5" => element = element.mt_5(),
//             "mt-6" => element = element.mt_6(),
//             "mt-8" => element = element.mt_8(),
//             "mt-10" => element = element.mt_10(),
//             "mt-12" => element = element.mt_12(),
//             "mt-16" => element = element.mt_16(),
//             "mt-20" => element = element.mt_20(),
//             "mt-24" => element = element.mt_24(),
//             "mt-32" => element = element.mt_32(),
//             "mt-40" => element = element.mt_40(),
//             "mt-48" => element = element.mt_48(),
//             "mt-56" => element = element.mt_56(),
//             "mt-64" => element = element.mt_64(),
//             "mt-72" => element = element.mt_72(),
//             "mt-80" => element = element.mt_80(),
//             "mt-96" => element = element.mt_96(),
//             "mt-px" => element = element.mt_px(),
//             "mt-1/2" => element = element.mt_1_2(),
//             "mt-1/3" => element = element.mt_1_3(),
//             "mt-2/3" => element = element.mt_2_3(),
//             "mt-1/4" => element = element.mt_1_4(),
//             "mt-2/4" => element = element.mt_2_4(),
//             "mt-3/4" => element = element.mt_3_4(),
//             "mt-1/5" => element = element.mt_1_5(),
//             "mt-2/5" => element = element.mt_2_5(),
//             "mt-3/5" => element = element.mt_3_5(),
//             "mt-4/5" => element = element.mt_4_5(),
//             "mt-1/6" => element = element.mt_1_6(),
//             "mt-5/6" => element = element.mt_5_6(),
//             "mt-1/12" => element = element.mt_1_12(),
//             "mr-0" => element = element.mr_0(),
//             "mr-1" => element = element.mr_1(),
//             "mr-2" => element = element.mr_2(),
//             "mr-3" => element = element.mr_3(),
//             "mr-4" => element = element.mr_4(),
//             "mr-5" => element = element.mr_5(),
//             "mr-6" => element = element.mr_6(),
//             "mr-8" => element = element.mr_8(),
//             "mr-10" => element = element.mr_10(),
//             "mr-12" => element = element.mr_12(),
//             "mr-16" => element = element.mr_16(),
//             "mr-20" => element = element.mr_20(),
//             "mr-24" => element = element.mr_24(),
//             "mr-32" => element = element.mr_32(),
//             "mr-40" => element = element.mr_40(),
//             "mr-48" => element = element.mr_48(),
//             "mr-56" => element = element.mr_56(),
//             "mr-64" => element = element.mr_64(),
//             "mr-72" => element = element.mr_72(),
//             "mr-80" => element = element.mr_80(),
//             "mr-96" => element = element.mr_96(),
//             "mr-px" => element = element.mr_px(),
//             "mr-1/2" => element = element.mr_1_2(),
//             "mr-1/3" => element = element.mr_1_3(),
//             "mr-2/3" => element = element.mr_2_3(),
//             "mr-1/4" => element = element.mr_1_4(),
//             "mr-2/4" => element = element.mr_2_4(),
//             "mr-3/4" => element = element.mr_3_4(),
//             "mr-1/5" => element = element.mr_1_5(),
//             "mr-2/5" => element = element.mr_2_5(),
//             "mr-3/5" => element = element.mr_3_5(),
//             "mr-4/5" => element = element.mr_4_5(),
//             "mr-1/6" => element = element.mr_1_6(),
//             "mr-5/6" => element = element.mr_5_6(),
//             "mr-1/12" => element = element.mr_1_12(),
//             "mb-0" => element = element.mb_0(),
//             "mb-1" => element = element.mb_1(),
//             "mb-2" => element = element.mb_2(),
//             "mb-3" => element = element.mb_3(),
//             "mb-4" => element = element.mb_4(),
//             "mb-5" => element = element.mb_5(),
//             "mb-6" => element = element.mb_6(),
//             "mb-8" => element = element.mb_8(),
//             "mb-10" => element = element.mb_10(),
//             "mb-12" => element = element.mb_12(),
//             "mb-16" => element = element.mb_16(),
//             "mb-20" => element = element.mb_20(),
//             "mb-24" => element = element.mb_24(),
//             "mb-32" => element = element.mb_32(),
//             "mb-40" => element = element.mb_40(),
//             "mb-48" => element = element.mb_48(),
//             "mb-56" => element = element.mb_56(),
//             "mb-64" => element = element.mb_64(),
//             "mb-72" => element = element.mb_72(),
//             "mb-80" => element = element.mb_80(),
//             "mb-96" => element = element.mb_96(),
//             "mb-px" => element = element.mb_px(),
//             "mb-1/2" => element = element.mb_1_2(),
//             "mb-1/3" => element = element.mb_1_3(),
//             "mb-2/3" => element = element.mb_2_3(),
//             "mb-1/4" => element = element.mb_1_4(),
//             "mb-2/4" => element = element.mb_2_4(),
//             "mb-3/4" => element = element.mb_3_4(),
//             "mb-1/5" => element = element.mb_1_5(),
//             "mb-2/5" => element = element.mb_2_5(),
//             "mb-3/5" => element = element.mb_3_5(),
//             "mb-4/5" => element = element.mb_4_5(),
//             "mb-1/6" => element = element.mb_1_6(),
//             "mb-5/6" => element = element.mb_5_6(),
//             "mb-1/12" => element = element.mb_1_12(),
//             "ml-0" => element = element.ml_0(),
//             "ml-1" => element = element.ml_1(),
//             "ml-2" => element = element.ml_2(),
//             "ml-3" => element = element.ml_3(),
//             "ml-4" => element = element.ml_4(),
//             "ml-5" => element = element.ml_5(),
//             "ml-6" => element = element.ml_6(),
//             "ml-8" => element = element.ml_8(),
//             "ml-10" => element = element.ml_10(),
//             "ml-12" => element = element.ml_12(),
//             "ml-16" => element = element.ml_16(),
//             "ml-20" => element = element.ml_20(),
//             "ml-24" => element = element.ml_24(),
//             "ml-32" => element = element.ml_32(),
//             "ml-40" => element = element.ml_40(),
//             "ml-48" => element = element.ml_48(),
//             "ml-56" => element = element.ml_56(),
//             "ml-64" => element = element.ml_64(),
//             "ml-72" => element = element.ml_72(),
//             "ml-80" => element = element.ml_80(),
//             "ml-96" => element = element.ml_96(),
//             "ml-px" => element = element.ml_px(),
//             "ml-1/2" => element = element.ml_1_2(),
//             "ml-1/3" => element = element.ml_1_3(),
//             "ml-2/3" => element = element.ml_2_3(),
//             "ml-1/4" => element = element.ml_1_4(),
//             "ml-2/4" => element = element.ml_2_4(),
//             "ml-3/4" => element = element.ml_3_4(),
//             "ml-1/5" => element = element.ml_1_5(),
//             "ml-2/5" => element = element.ml_2_5(),
//             "ml-3/5" => element = element.ml_3_5(),
//             "ml-4/5" => element = element.ml_4_5(),
//             "ml-1/6" => element = element.ml_1_6(),
//             "ml-5/6" => element = element.ml_5_6(),
//             "ml-1/12" => element = element.ml_1_12(),
//             _ => { /* ignore unknown attributes */ }
//         }
//     }

//     element
// }

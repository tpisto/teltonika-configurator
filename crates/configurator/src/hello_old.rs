use gpui::*;

pub struct HelloWorld {
    pub text: SharedString,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            text: "Hello, World!".into(),
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut root = div().flex().size_full();
        let childs = vec!["a", "b"];

        let child_elements: Vec<_> = childs
            .into_iter()
            .map(|child| {
                // Create each child element
                div()
                    .flex()
                    .size_full()
                    .bg(rgb(0x0000ff))
                    .child(format!("Hello world: , {}!", &self.text))
            })
            .collect();

        root = root.children(child_elements);

        root
    }
}

use gpui::*;

#[derive(Clone, IntoElement)]
pub struct InputCheckbox {}

impl InputCheckbox {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for InputCheckbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div().h_10().w_20().m_1().bg(rgb(0x0000ff))
    }
}

impl Styled for InputCheckbox {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.style()
    }
}

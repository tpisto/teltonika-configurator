use gpui::*;

#[derive(Clone, IntoElement)]
pub struct InputNumber {}

impl InputNumber {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for InputNumber {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div().h_10().w_20().m_1().bg(rgb(0x0000ff))
    }
}

impl Styled for InputNumber {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.style()
    }
}

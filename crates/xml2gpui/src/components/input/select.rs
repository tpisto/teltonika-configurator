use gpui::*;

#[derive(Clone, IntoElement)]
pub struct InputSelect {}

impl InputSelect {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for InputSelect {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div().h_10().w_20().m_1().bg(rgb(0x00ffff))
    }
}

impl Styled for InputSelect {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.style()
    }
}

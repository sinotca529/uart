use tui::widgets::Widget;

pub struct OnetimeWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    render: F,
}

impl<F> OnetimeWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    pub fn new(render: F) -> Self {
        Self { render }
    }
}

impl<F> Widget for OnetimeWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        (self.render)(area, buf);
    }
}

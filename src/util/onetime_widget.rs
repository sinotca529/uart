use ratatui::widgets::Widget;

pub struct OnetimeWidget<F>
where
    F: FnOnce(ratatui::layout::Rect, &mut ratatui::buffer::Buffer),
{
    render: F,
}

impl<F> OnetimeWidget<F>
where
    F: FnOnce(ratatui::layout::Rect, &mut ratatui::buffer::Buffer),
{
    pub fn new(render: F) -> Self {
        Self { render }
    }
}

impl<F> Widget for OnetimeWidget<F>
where
    F: FnOnce(ratatui::layout::Rect, &mut ratatui::buffer::Buffer),
{
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        (self.render)(area, buf);
    }
}

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Widget to render command line.
pub struct CmdLine<'a>(Paragraph<'a>);

impl<'a> CmdLine<'a> {
    pub fn new(p: Paragraph<'a>) -> Self {
        Self(p)
    }
}

impl<'a> Widget for CmdLine<'a> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let cmd_line = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(50, 50, 50)));
        self.0.block(cmd_line).render(area, buf);
    }
}

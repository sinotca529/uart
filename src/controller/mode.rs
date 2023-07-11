use self::normal::NormalMode;

use super::AppOp;
use crate::{canvas::shape::Shape, util::Coord};
use crossterm::event::Event;
use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

mod command;
mod make_rect;
mod make_text;
pub mod normal;

pub trait Mode {
    fn next(self: Box<Self>, e: Event, canvas_cursor: Coord) -> (Box<dyn Mode>, AppOp);
    fn additional_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        vec![]
    }
    fn status_msg(&self) -> Paragraph;
}

impl Default for Box<dyn Mode> {
    fn default() -> Self {
        Box::new(NormalMode)
    }
}

impl Widget for &Box<dyn Mode> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let cmd_line = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(50, 50, 50)));
        self.status_msg().block(cmd_line).render(area, buf);
    }
}

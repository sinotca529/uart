use crossterm::event::Event;
use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use self::{
    command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, normal::NormalMode,
};
use crate::{canvas::shape::Shape, util::Coord};

use super::AppOp;

mod command;
mod make_rect;
mod make_text;
mod normal;

trait ModeIf {
    fn next(self, e: Event, canvas_cursor: Coord) -> (Mode, AppOp);
    fn additional_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Shape)> {
        vec![]
    }
    fn status_msg(&self) -> Paragraph;
}

pub enum Mode {
    Norm(NormalMode),
    Cmd(CmdMode),
    MakeRect(MakeRectMode),
    MakeText(MakeTextMode),
}

impl Mode {
    pub fn new() -> Self {
        Self::Norm(NormalMode::new())
    }

    pub fn additional_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Shape)> {
        let m: &dyn ModeIf = self.into();
        m.additional_shapes(canvas_cursor)
    }

    pub fn trans(&mut self, e: Event, canvas_cursor: Coord) -> AppOp {
        // TODO refact
        let app_op;
        let mut old = Self::new();
        std::mem::swap(self, &mut old);

        (*self, app_op) = match old {
            Mode::Norm(m) => m.next(e, canvas_cursor),
            Mode::Cmd(m) => m.next(e, canvas_cursor),
            Mode::MakeRect(m) => m.next(e, canvas_cursor),
            Mode::MakeText(m) => m.next(e, canvas_cursor),
        };
        app_op
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &Mode {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let cmd_line = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(50, 50, 50)));
        let m: &dyn ModeIf = self.into();
        m.status_msg().block(cmd_line).render(area, buf);
    }
}

impl<'a> From<&'a Mode> for &'a dyn ModeIf {
    fn from(val: &'a Mode) -> Self {
        match val {
            Mode::Norm(m) => m,
            Mode::Cmd(m) => m,
            Mode::MakeRect(m) => m,
            Mode::MakeText(m) => m,
        }
    }
}

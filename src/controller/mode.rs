use crossterm::event::Event;
use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use self::{
    command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, normal::NormalMode,
};
use crate::util::Coord;

use super::AppOp;

mod command;
mod make_rect;
mod make_text;
mod normal;

trait ModeIf {
    fn canvas_cursor(&self) -> &Coord;
    fn next(self, e: Event) -> (Mode, AppOp);
    fn modify_canvas_view(&self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer) {}
    fn status_msg(&self) -> Paragraph;
    fn render_cursor(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let Coord { x, y } = self.canvas_cursor();
        buf.get_mut(area.x + x, area.y + y)
            .set_bg(Color::Rgb(128, 128, 128));
    }
}

pub enum Mode {
    Norm(NormalMode),
    Cmd(CmdMode),
    MakeRect(MakeRectMode),
    MakeText(MakeTextMode),
}

impl Mode {
    pub fn new() -> Self {
        Self::Norm(NormalMode::new(Coord::new(0, 0)))
    }

    pub fn canvas_modify_widget(&self) -> CanvasModifyWidget {
        CanvasModifyWidget { mode: self.into() }
    }

    pub fn trans(&mut self, e: Event) -> AppOp {
        // TODO refact
        let mut old = Self::new();
        std::mem::swap(self, &mut old);

        let (next_mode, app_op) = match old {
            Mode::Norm(m) => m.next(e),
            Mode::Cmd(m) => m.next(e),
            Mode::MakeRect(m) => m.next(e),
            Mode::MakeText(m) => m.next(e),
        };

        *self = next_mode;
        app_op
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

pub struct CanvasModifyWidget<'a> {
    mode: &'a dyn ModeIf,
}

impl<'a> Widget for CanvasModifyWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        self.mode.modify_canvas_view(area, buf)
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

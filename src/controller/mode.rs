use crossterm::event::Event;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
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

    fn inner(&self) -> &dyn ModeIf {
        match self {
            Mode::Norm(m) => m,
            Mode::Cmd(m) => m,
            Mode::MakeRect(m) => m,
            Mode::MakeText(m) => m,
        }
    }

    pub fn canvas_modify_widget(&self) -> CanvasModifyWidget {
        CanvasModifyWidget { mode: self.inner() }
    }

    pub fn canvas_cursor(&self) -> &Coord {
        self.inner().canvas_cursor()
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
        // todo refact
        let text;
        let fg_color;

        match self {
            Mode::Norm(_) => {
                text = "Begin command by ':'";
                fg_color = Color::Rgb(128, 128, 128);
            }
            Mode::Cmd(c) => {
                text = c.cmd();
                fg_color = Color::White;
            }
            Mode::MakeRect(_) => {
                text = "Make rect with Enter";
                fg_color = Color::White;
            }
            Mode::MakeText(_) => {
                text = "Make text with Esc";
                fg_color = Color::White;
            }
        }

        let cmd_line = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(50, 50, 50)));
        let msg = Paragraph::new(Text::raw(text))
            .block(cmd_line)
            .style(Style::default().fg(fg_color).bg(Color::Rgb(50, 50, 50)))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });
        msg.render(area, buf);
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

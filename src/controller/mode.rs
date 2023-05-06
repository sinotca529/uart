use crossterm::event::Event;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use self::{command::CmdMode, normal::NormalMode};
use crate::util::Coord;

use super::AppOp;

pub mod command;
pub mod normal;

pub enum Mode {
    Norm(NormalMode),
    Cmd(CmdMode),
}

impl Mode {
    pub fn new() -> Self {
        Self::Norm(NormalMode::new(Coord::new(0, 0)))
    }

    pub fn canvas_cursor(&self) -> &Coord {
        match self {
            Mode::Norm(m) => m.canvas_cursor(),
            Mode::Cmd(m) => m.canvas_cursor(),
        }
    }

    pub fn trans(&mut self, e: Event) -> AppOp {
        let mut old = Self::new();
        std::mem::swap(self, &mut old);

        let (next_mode, app_op) = match old {
            Mode::Norm(m) => m.next(e),
            Mode::Cmd(m) => m.next(e),
        };

        *self = next_mode;
        app_op
    }
}

impl Widget for &Mode {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
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

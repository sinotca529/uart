use super::{normal::NormalMode, Mode};
use crate::app::{canvas::cursor::Cursor, AppOp};
use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

/// Operations for command mode.
enum Op {
    /// Enter key is pressed.
    Enter,
    /// Char key is pressed.
    Char(char),
    /// BS key is pressed.
    BackSpace,
    /// Do nothing.
    Nop,
}

impl From<Event> for Op {
    fn from(val: Event) -> Self {
        match val {
            Event::Key(k) => match k.code {
                KeyCode::Backspace => Op::BackSpace,
                KeyCode::Enter => Op::Enter,
                KeyCode::Char(c) => Op::Char(c),
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct CmdMode {
    cmd: String,
}

impl CmdMode {
    pub fn new() -> Self {
        Self {
            cmd: ":".to_string(),
        }
    }
}

impl Default for CmdMode {
    fn default() -> Self {
        Self::new()
    }
}

impl Mode for CmdMode {
    fn next(mut self: Box<Self>, e: Event, _: &Cursor) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::Enter => {
                let app_op = if self.cmd == ":q" {
                    AppOp::QuitApp
                } else {
                    AppOp::Nop
                };

                let next_mode = Box::new(NormalMode::new());
                (next_mode, app_op)
            }
            Op::Char(c) => {
                self.cmd.push(c);
                (self, AppOp::Nop)
            }
            Op::BackSpace => {
                self.cmd.pop();
                if self.cmd.is_empty() {
                    (Box::new(NormalMode::new()), AppOp::Nop)
                } else {
                    (self, AppOp::Nop)
                }
            }
            Op::Nop => (self, AppOp::Nop),
        }
    }

    fn status_msg(&self) -> tui::widgets::Paragraph {
        let t = tui::text::Text::raw(self.cmd.clone());
        Paragraph::new(t)
            .style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(50, 50, 50)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
    }
}

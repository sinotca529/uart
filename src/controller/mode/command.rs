use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

use super::{normal::NormalMode, Mode, ModeIf};
use crate::{controller::AppOp, util::Coord};

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
    canvas_cursor: Coord,
    cmd: String,
}

impl CmdMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            canvas_cursor,
            cmd: ":".to_string(),
        }
    }

    pub fn cmd(&self) -> &String {
        &self.cmd
    }
}

impl ModeIf for CmdMode {
    fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    fn next(mut self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::Enter => {
                let app_op = if self.cmd == ":q" {
                    AppOp::QuitApp
                } else {
                    AppOp::Nop
                };

                let next_mode = NormalMode::new(self.canvas_cursor).into();
                (next_mode, app_op)
            }
            Op::Char(c) => {
                self.cmd.push(c);
                (self.into(), AppOp::Nop)
            }
            Op::BackSpace => {
                self.cmd.pop();
                let next_mode = if self.cmd.is_empty() {
                    NormalMode::new(self.canvas_cursor).into()
                } else {
                    self.into()
                };
                (next_mode, AppOp::Nop)
            }
            Op::Nop => (self.into(), AppOp::Nop),
        }
    }

    fn modify_canvas_view(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // draw cursor
        let Coord { x, y } = self.canvas_cursor;
        buf.get_mut(area.x + x, area.y + y)
            .set_bg(Color::Rgb(128, 128, 128));
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

impl From<CmdMode> for Mode {
    fn from(val: CmdMode) -> Self {
        Mode::Cmd(val)
    }
}

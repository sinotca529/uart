use super::{command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, Mode, ModeIf};
use crate::{
    controller::AppOp,
    util::{Coord, Direction},
};
use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

/// Operations for normal mode.
enum Op {
    /// Change to cmd mode.
    EnterCmd,
    /// Change to make rect mode.
    EnterMakeRect,
    /// Change to make text mode.
    EnterMakeText,
    /// Move Cursor
    MoveCursor(Direction),
    /// Do nothing.
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => match c {
                    ':' => Op::EnterCmd,
                    'h' => Op::MoveCursor(Direction::Left),
                    'j' => Op::MoveCursor(Direction::Down),
                    'k' => Op::MoveCursor(Direction::Up),
                    'l' => Op::MoveCursor(Direction::Right),
                    'r' => Op::EnterMakeRect,
                    't' => Op::EnterMakeText,
                    _ => Op::Nop,
                },
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct NormalMode;

impl NormalMode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NormalMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeIf for NormalMode {
    fn next(self, e: Event, canvas_cursor: Coord) -> (Mode, AppOp) {
        match e.into() {
            Op::EnterCmd => {
                let cmd = CmdMode::new().into();
                (cmd, AppOp::Nop)
            }
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MoveCursor(d) => (self.into(), AppOp::MoveCanvasCursor(d)),
            Op::EnterMakeRect => (MakeRectMode::new(canvas_cursor).into(), AppOp::Nop),
            Op::EnterMakeText => (MakeTextMode::new(canvas_cursor).into(), AppOp::Nop),
        }
    }

    fn status_msg(&self) -> tui::widgets::Paragraph {
        let t = tui::text::Text::raw("NORM [:]cmd [r]rect [t]text");
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

impl From<NormalMode> for Mode {
    fn from(val: NormalMode) -> Self {
        Mode::Norm(val)
    }
}

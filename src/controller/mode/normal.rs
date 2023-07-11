use super::{command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, Mode};
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

impl Mode for NormalMode {
    fn next(self: Box<Self>, e: Event, canvas_cursor: Coord) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::EnterCmd => {
                let cmd = Box::new(CmdMode::new());
                (cmd, AppOp::Nop)
            }
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::EnterMakeRect => (Box::new(MakeRectMode::new(canvas_cursor)), AppOp::Nop),
            Op::EnterMakeText => (Box::new(MakeTextMode::new(canvas_cursor)), AppOp::Nop),
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

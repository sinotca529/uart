use super::{command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, Mode, ModeIf};
use crate::{
    controller::AppOp,
    util::{Coord, Direction},
};
use crossterm::event::{Event, KeyCode};
use tui::style::Color;

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

pub struct NormalMode {
    canvas_cursor: Coord,
}

impl NormalMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self { canvas_cursor }
    }
}

impl ModeIf for NormalMode {
    fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    fn next(mut self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::EnterCmd => {
                let cmd = CmdMode::new(self.canvas_cursor).into();
                (cmd, AppOp::Nop)
            }
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MoveCursor(d) => {
                self.canvas_cursor = self.canvas_cursor.adjacency(d);
                (self.into(), AppOp::Nop)
            }
            Op::EnterMakeRect => (MakeRectMode::new(self.canvas_cursor).into(), AppOp::Nop),
            Op::EnterMakeText => (MakeTextMode::new(self.canvas_cursor).into(), AppOp::Nop),
        }
    }

    fn modify_canvas_view(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // draw cursor
        let Coord { x, y } = self.canvas_cursor;
        buf.get_mut(area.x + x, area.y + y)
            .set_bg(Color::Rgb(128, 128, 128));
    }
}

impl From<NormalMode> for Mode {
    fn from(val: NormalMode) -> Self {
        Mode::Norm(val)
    }
}

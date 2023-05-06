use super::{command::CmdMode, Mode};
use crate::{
    controller::AppOp,
    util::{Coord, Direction},
};
use crossterm::event::{Event, KeyCode};

/// Operations for normal mode.
enum Op {
    /// Change to cmd mode.
    EnterCmd,
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

    pub fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    pub fn next(mut self, e: Event) -> (Mode, AppOp) {
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
        }
    }
}

impl From<NormalMode> for Mode {
    fn from(val: NormalMode) -> Self {
        Mode::Norm(val)
    }
}

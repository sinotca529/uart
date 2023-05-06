use super::{command::CmdMode, Mode};
use crate::{util::Coord, controller::AppOp};
use crossterm::event::{Event, KeyCode};

/// Operations for normal mode.
enum Op {
    /// Change to cmd mode.
    EnterCmd,
    /// Do nothing.
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => {
                if k.code == KeyCode::Char(':') {
                    Op::EnterCmd
                } else {
                    Op::Nop
                }
            }
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

    pub fn next(self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::EnterCmd => {
                let cmd = CmdMode::new(self.canvas_cursor).into();
                (cmd, AppOp::Nop)
            }
            Op::Nop => (self.into(), AppOp::Nop),
        }
    }
}

impl From<NormalMode> for Mode {
    fn from(val: NormalMode) -> Self {
        Mode::Norm(val)
    }
}

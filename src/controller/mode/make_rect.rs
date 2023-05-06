use crossterm::event::{Event, KeyCode};

use crate::{
    canvas::shape::rect::Rect,
    controller::AppOp,
    util::{Coord, Direction, Size},
};

use super::{normal::NormalMode, Mode};

enum Op {
    MoveCursor(Direction),
    MakeRect,
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Enter => Op::MakeRect,
                KeyCode::Char(c) => match c {
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

pub struct MakeRectMode {
    canvas_cursor: Coord,
    start_coord: Coord,
}

impl MakeRectMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            canvas_cursor,
            start_coord: canvas_cursor,
        }
    }

    pub fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    pub fn next(mut self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MoveCursor(d) => {
                self.canvas_cursor = self.canvas_cursor.adjacency(d);
                (self.into(), AppOp::Nop)
            }
            Op::MakeRect => {
                let w = self.start_coord.x.abs_diff(self.canvas_cursor.x) + 1;
                let h = self.start_coord.y.abs_diff(self.canvas_cursor.y) + 1;
                let rect = Rect::new(Size::new(w, h));

                let x = self.start_coord.x.min(self.canvas_cursor.x);
                let y = self.start_coord.y.min(self.canvas_cursor.y);
                let start = Coord::new(x, y);

                let op = AppOp::MakeRect(start, rect);

                let mode = NormalMode::new(self.canvas_cursor).into();
                (mode, op)
            }
        }
    }
}

impl From<MakeRectMode> for Mode {
    fn from(val: MakeRectMode) -> Self {
        Mode::MakeRect(val)
    }
}

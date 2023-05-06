use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::{
    canvas::shape::{text::Text, ShapeIf, ShapeWithCoord},
    controller::AppOp,
    util::{make_area, Coord, Direction},
};

use super::{normal::NormalMode, Mode, ModeIf};

enum Op {
    MakeText,
    AddChar(char),
    Enter,
    Backspace,
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Enter => Op::Enter,
                KeyCode::Char(c) => Op::AddChar(c),
                KeyCode::Backspace => Op::Backspace,
                KeyCode::Esc => Op::MakeText,
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct MakeTextMode {
    canvas_cursor: Coord,
    start_coord: Coord,
    text: String,
}

impl MakeTextMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            canvas_cursor,
            start_coord: canvas_cursor,
            text: String::new(),
        }
    }
}

impl ModeIf for MakeTextMode {
    fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    fn next(mut self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MakeText => {
                let mode = NormalMode::new(self.canvas_cursor).into();
                let op = AppOp::MakeShape(self.start_coord, Text::new(self.text.clone()).into());
                (mode, op)
            }
            Op::AddChar(c) => {
                self.text.push(c);
                self.canvas_cursor = self.canvas_cursor.adjacency(Direction::Right);
                (self.into(), AppOp::Nop)
            }
            Op::Enter => {
                self.text.push('\n');
                self.canvas_cursor = self.canvas_cursor.adjacency(Direction::Down);
                (self.into(), AppOp::Nop)
            }
            Op::Backspace => {
                let c = self.text.pop();
                // Todo update cursor pos
                // match c {
                //     Some('\n') => {
                //         self.canvas_cursor = self.canvas_cursor.adjacency(Direction::Up);
                //     },
                //     Some(c) => {},
                //     _ => {},
                // }
                (self.into(), AppOp::Nop)
            }
        }
    }

    fn modify_canvas_view(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // draw text
        let text = Text::new(self.text.clone());
        ShapeWithCoord::new(&text, &self.start_coord).render(area, buf);
    }
}

impl From<MakeTextMode> for Mode {
    fn from(val: MakeTextMode) -> Self {
        Mode::MakeText(val)
    }
}

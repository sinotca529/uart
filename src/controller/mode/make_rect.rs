use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

use crate::{
    canvas::shape::{rect::Rect, Shape},
    controller::AppOp,
    util::{Coord, Direction, Size},
};

use super::{normal::NormalMode, Mode, ModeIf};

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
    start_coord: Coord,
}

impl MakeRectMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
        }
    }

    /// Make rect
    fn make_rect(a: Coord, b: Coord) -> (Coord, Rect) {
        let w = a.x.abs_diff(b.x) + 1;
        let h = a.y.abs_diff(b.y) + 1;
        let rect = Rect::new(Size::new(w, h));

        let x = a.x.min(b.x);
        let y = a.y.min(b.y);
        let start = Coord::new(x, y);

        (start, rect)
    }
}

impl ModeIf for MakeRectMode {
    fn next(self, e: Event, canvas_cursor: Coord) -> (Mode, AppOp) {
        match e.into() {
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MoveCursor(d) => (self.into(), AppOp::MoveCanvasCursor(d)),
            Op::MakeRect => {
                let (start, rect) = Self::make_rect(self.start_coord, canvas_cursor);
                let op = AppOp::MakeShape(start, rect.into());
                let mode = NormalMode.into();
                (mode, op)
            }
        }
    }

    fn additional_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Shape)> {
        let (start, rect) = Self::make_rect(self.start_coord, canvas_cursor);
        vec![(start, rect.into())]
    }

    fn status_msg(&self) -> tui::widgets::Paragraph {
        let t = tui::text::Text::raw("RECT [Enter]Complete");
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

impl From<MakeRectMode> for Mode {
    fn from(val: MakeRectMode) -> Self {
        Mode::MakeRect(val)
    }
}

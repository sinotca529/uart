use crate::{
    app::{
        canvas::cursor::Cursor,
        shape::{rect::Rect, Shape},
        AppOp,
    },
    util::{Coord, Direction, Size},
};
use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
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

impl Mode for MakeRectMode {
    fn next(self: Box<Self>, e: Event, cursor: &Cursor) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::MakeRect => {
                let (start, rect) = Self::make_rect(self.start_coord, cursor.coord());
                let op = AppOp::MakeShape(start, Box::new(rect));
                let mode = Box::new(NormalMode);
                (mode, op)
            }
        }
    }

    fn additinal_canvas_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let (start, rect) = Self::make_rect(self.start_coord, canvas_cursor);
        vec![(start, Box::new(rect))]
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

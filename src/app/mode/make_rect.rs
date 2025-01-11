use crate::{
    app::{
        canvas::CanvasHandler,
        shape::{rect::Rect, style::Style, Shape},
        AppOp,
    },
    util::{Coord, Direction, Size},
};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Paragraph, Wrap},
};

use super::{normal::NormalMode, Mode};

enum Op {
    MoveCursor(Direction),
    MakeRect,
    NextStyle,
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
                    's' => Op::NextStyle,
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
    style: Style,
}

impl MakeRectMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            style: Style::Single,
        }
    }

    /// Make rect
    fn make_rect(a: Coord, b: Coord, style: Style) -> (Coord, Rect) {
        let w = a.x.abs_diff(b.x) + 1;
        let h = a.y.abs_diff(b.y) + 1;
        let rect = Rect::new(Size::new(w, h), style);

        let x = a.x.min(b.x);
        let y = a.y.min(b.y);
        let start = Coord::new(x, y);

        (start, rect)
    }
}

impl Mode for MakeRectMode {
    fn next(mut self: Box<Self>, e: Event, canvas_handler: &CanvasHandler) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::NextStyle => {
                self.style = self.style.next();
                (self, AppOp::Nop)
            }
            Op::MakeRect => {
                let (start, rect) =
                    Self::make_rect(self.start_coord, canvas_handler.cursor_coord(), self.style);
                let op = AppOp::MakeShape(start, Box::new(rect));
                let mode = Box::new(NormalMode);
                (mode, op)
            }
        }
    }

    fn additinal_canvas_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let (start, rect) = Self::make_rect(self.start_coord, canvas_cursor, self.style);
        vec![(start, Box::new(rect))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("RECT [Enter]Complete, [s]Change Line Style");
        Paragraph::new(t)
            .style(
                ratatui::style::Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(50, 50, 50)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
    }
}

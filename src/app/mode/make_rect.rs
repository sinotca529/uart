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
    rect: Rect,
}

impl MakeRectMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            rect: Rect::new(Size::new(1, 1), Style::Single),
        }
    }

    fn upper_left_corner(&self, current_cursor: Coord) -> Coord {
        Coord::new(
            self.start_coord.x.min(current_cursor.x),
            self.start_coord.y.min(current_cursor.y),
        )
    }

    fn update_rect_size(&mut self, current_cursor: Coord) {
        let diff = current_cursor - self.start_coord;
        let (w, h) = (diff.x.abs() as u16 + 1, diff.y.abs() as u16 + 1);
        self.rect.set_size(Size::new(w, h));
    }
}

impl Mode for MakeRectMode {
    fn next(
        mut self: Box<Self>,
        e: Event,
        canvas_handler: &CanvasHandler,
    ) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => {
                self.update_rect_size(canvas_handler.cursor_coord().adjacency(d));
                (self, AppOp::MoveCanvasCursor(d))
            }
            Op::NextStyle => {
                self.rect.set_next_line_style();
                (self, AppOp::Nop)
            }
            Op::MakeRect => {
                let upper_left = self.upper_left_corner(canvas_handler.cursor_coord());
                let op = AppOp::MakeShape(upper_left, Box::new(self.rect.clone()));
                (Box::new(NormalMode), op)
            }
        }
    }

    fn additinal_canvas_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let upper_left = self.upper_left_corner(canvas_cursor);
        vec![(upper_left, Box::new(self.rect.clone()))]
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

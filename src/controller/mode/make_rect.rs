use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::{
    canvas::shape::{rect::Rect, ShapeWithCoord},
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

    /// Make rect
    fn make_rect(&self) -> (Coord, Rect) {
        let w = self.start_coord.x.abs_diff(self.canvas_cursor.x) + 1;
        let h = self.start_coord.y.abs_diff(self.canvas_cursor.y) + 1;
        let rect = Rect::new(Size::new(w, h));

        let x = self.start_coord.x.min(self.canvas_cursor.x);
        let y = self.start_coord.y.min(self.canvas_cursor.y);
        let start = Coord::new(x, y);

        (start, rect)
    }
}

impl ModeIf for MakeRectMode {
    fn canvas_cursor(&self) -> &Coord {
        &self.canvas_cursor
    }

    fn next(mut self, e: Event) -> (Mode, AppOp) {
        match e.into() {
            Op::Nop => (self.into(), AppOp::Nop),
            Op::MoveCursor(d) => {
                self.canvas_cursor = self.canvas_cursor.adjacency(d);
                (self.into(), AppOp::Nop)
            }
            Op::MakeRect => {
                let (start, rect) = self.make_rect();
                let op = AppOp::MakeShape(start, rect.into());
                let mode = NormalMode::new(self.canvas_cursor).into();
                (mode, op)
            }
        }
    }

    fn modify_canvas_view(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // draw rect
        let (start, rect) = self.make_rect();
        ShapeWithCoord::new(&rect, &start).render(area, buf);

        // draw cursor
        let Coord { x, y } = self.canvas_cursor;
        buf.get_mut(area.x + x, area.y + y)
            .set_bg(Color::Rgb(128, 128, 128));
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

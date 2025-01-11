use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Paragraph, Wrap},
};

use crate::{
    app::{
        shape::{path::Path, style::Style, Shape},
        AppOp,
    },
    util::{Coord, Direction},
};

use super::{normal::NormalMode, Mode};

enum Op {
    MoveCursor(Direction),
    /// Pop one step from path
    Back,
    MakePath,
    SelectNextStyle,
    SelectNextArrowState,
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Enter => Op::MakePath,
                KeyCode::Char(c) => match c {
                    'h' => Op::MoveCursor(Direction::Left),
                    'j' => Op::MoveCursor(Direction::Down),
                    'k' => Op::MoveCursor(Direction::Up),
                    'l' => Op::MoveCursor(Direction::Right),
                    's' => Op::SelectNextStyle,
                    'a' => Op::SelectNextArrowState,
                    _ => Op::Nop,
                },
                KeyCode::Backspace => Op::Back,
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct MakePathMode {
    start_coord: Coord,
    path: Path,
}

impl MakePathMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            path: Path::new(vec![], false, false, Style::Single),
        }
    }
}

impl Mode for MakePathMode {
    fn next(
        mut self: Box<Self>,
        e: Event,
        _canvas_handler: &crate::app::canvas::CanvasHandler,
    ) -> (Box<dyn Mode>, crate::app::AppOp) {
        match e.into() {
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => {
                self.path.push_path(d);
                (self, AppOp::MoveCanvasCursor(d))
            }
            Op::Back => match self.path.pop_path() {
                Some(dir) => (self, AppOp::MoveCanvasCursor(dir.opposite())),
                None => (self, AppOp::Nop),
            },
            Op::MakePath => {
                let op = if self.path.is_empty() {
                    AppOp::Nop
                } else {
                    let upper_left = self.start_coord + self.path.start_to_upper_left();
                    AppOp::MakeShape(upper_left, Box::new(self.path.clone()))
                };
                let mode = Box::new(NormalMode);
                (mode, op)
            }
            Op::SelectNextStyle => {
                self.path.set_next_line_style();
                (self, AppOp::Nop)
            }
            Op::SelectNextArrowState => {
                self.path.set_next_arrow_state();
                (self, AppOp::Nop)
            }
        }
    }

    fn additinal_canvas_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let start = self.start_coord + self.path.start_to_upper_left();
        vec![(start, Box::new(self.path.clone()))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("LINE [Enter]Complete, [s]Change Line Style");
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

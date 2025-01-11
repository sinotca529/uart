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
    line_style: Style,
    path: Vec<Direction>,
}

impl MakePathMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            path: vec![],
            line_style: Style::Single,
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
                self.path.push(d);
                (self, AppOp::MoveCanvasCursor(d))
            }
            Op::Back => {
                if self.path.is_empty() {
                    return (self, AppOp::Nop);
                }

                let dir = self.path.pop().unwrap();
                (self, AppOp::MoveCanvasCursor(dir.opposite()))
            }
            Op::MakePath => {
                let mode = Box::new(NormalMode);
                if self.path.is_empty() {
                    return (mode, AppOp::Nop);
                }

                let line = Path::new(self.path.clone(), false, false, self.line_style);
                let start = self.start_coord + line.start_to_upper_left();
                let op = AppOp::MakeShape(start, Box::new(line));
                (mode, op)
            }
            Op::SelectNextStyle => {
                self.line_style = self.line_style.next();
                (self, AppOp::Nop)
            }
        }
    }

    fn additinal_canvas_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let line = Path::new(self.path.clone(), false, false, self.line_style);
        let start = self.start_coord + line.start_to_upper_left();
        vec![(start, Box::new(line))]
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

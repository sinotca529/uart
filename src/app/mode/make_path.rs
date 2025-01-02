use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

use crate::{
    app::{
        shape::{path::Path, Shape},
        AppOp,
    },
    util::{Coord, Direction},
};

use super::{normal::NormalMode, Mode};

enum Op {
    MoveCursor(Direction),
    MakePath,
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
                    _ => Op::Nop,
                },
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct MakePathMode {
    start_coord: Coord,
    path: Vec<Direction>,
}

impl MakePathMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            path: vec![],
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
            Op::MakePath => {
                let line = Path::new(self.path.clone(), false, false);
                let start = self.start_coord + line.start_to_upper_left();
                let op = AppOp::MakeShape(start, Box::new(line));
                let mode = Box::new(NormalMode);
                (mode, op)
            }
        }
    }

    fn additinal_canvas_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let line = Path::new(self.path.clone(), false, false);
        let start = self.start_coord + line.start_to_upper_left();
        vec![(start, Box::new(line))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("LINE [Enter]Complete");
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

use super::{normal::NormalMode, Mode};
use crate::{
    app::{canvas::CanvasHandler, AppOp},
    util::Direction,
};
use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

enum Op {
    ToggleSelect,
    MoveCursor(Direction),
    MoveShapes(Direction),
    DeleteShapes,
    EnterNormalMode,
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Esc => Op::EnterNormalMode,
                KeyCode::Char(c) => match c {
                    ' ' => Op::ToggleSelect,
                    'd' => Op::DeleteShapes,
                    'h' => Op::MoveCursor(Direction::Left),
                    'j' => Op::MoveCursor(Direction::Down),
                    'k' => Op::MoveCursor(Direction::Up),
                    'l' => Op::MoveCursor(Direction::Right),
                    'H' => Op::MoveShapes(Direction::Left),
                    'J' => Op::MoveShapes(Direction::Down),
                    'K' => Op::MoveShapes(Direction::Up),
                    'L' => Op::MoveShapes(Direction::Right),
                    _ => Op::Nop,
                },
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct SelectMode;

impl SelectMode {
    pub fn new() -> Self {
        Self
    }
}

impl Mode for SelectMode {
    fn next(
        self: Box<Self>,
        e: Event,
        canvas_hanler: &CanvasHandler,
    ) -> (Box<dyn Mode>, crate::app::AppOp) {
        match e.into() {
            Op::ToggleSelect => {
                if canvas_hanler.will_toggle_last_selected_shape() {
                    (Box::new(NormalMode::new()), AppOp::UnselectAllShape)
                } else {
                    (self, AppOp::ToggleShapeSelect)
                }
            }
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::MoveShapes(d) => (self, AppOp::MoveSlectedShapes(d)),
            Op::DeleteShapes => (Box::new(NormalMode::new()), AppOp::DeleteSelectedShapes),
            Op::EnterNormalMode => (Box::new(NormalMode::new()), AppOp::UnselectAllShape),
            Op::Nop => (self, AppOp::Nop),
        }
    }

    fn status_msg(&self) -> tui::widgets::Paragraph {
        let t = tui::text::Text::raw("SELECT [sp]toggle select [d]delete [S-h/j/k/l]move");
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

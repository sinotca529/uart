use super::{normal::NormalMode, Mode};
use crate::{
    app::{
        canvas::{CanvasHandler, ShapeId, ShapeIdSet},
        AppOp,
    },
    util::Direction,
};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

enum Op {
    ToggleSelect(ShapeId),
    MoveCursor(Direction),
    MoveShapes(Direction),
    DeleteShapes,
    EnterNormalMode,
    Nop,
}

impl From<(Event, &CanvasHandler)> for Op {
    fn from((e, ch): (Event, &CanvasHandler)) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Esc => Op::EnterNormalMode,
                KeyCode::Char(c) => match c {
                    ' ' => match ch.shape_id_under_the_cursor() {
                        Some(id) => Op::ToggleSelect(id),
                        None => Op::Nop,
                    },
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

pub struct SelectMode {
    selected_shapes: ShapeIdSet,
}

impl SelectMode {
    /// id: initial selected shape
    pub fn new(id: ShapeId) -> Self {
        let mut selected_shapes = ShapeIdSet::default();
        selected_shapes.insert(&id);
        Self { selected_shapes }
    }
}

impl Mode for SelectMode {
    fn next(
        mut self: Box<Self>,
        e: Event,
        canvas_hanler: &CanvasHandler,
    ) -> (Box<dyn Mode>, crate::app::AppOp) {
        match (e, canvas_hanler).into() {
            Op::ToggleSelect(id) => {
                self.selected_shapes.toggle(&id);
                if self.selected_shapes.is_empty() {
                    (Box::new(NormalMode::new()), AppOp::Nop)
                } else {
                    (self, AppOp::Nop)
                }
            }
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::MoveShapes(d) => {
                let shapes = self.selected_shapes.clone();
                (self, AppOp::MoveShapesAndCanvasCursor(shapes, d))
            }
            Op::DeleteShapes => (
                Box::new(NormalMode::new()),
                AppOp::DeleteShapes(self.selected_shapes),
            ),
            Op::EnterNormalMode => (Box::new(NormalMode::new()), AppOp::Nop),
            Op::Nop => (self, AppOp::Nop),
        }
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("SELECT [ ]toggle select [d]delete [HJKL]move");
        Paragraph::new(t)
            .style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(50, 50, 50)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
    }

    fn shapes_to_highlight(&self) -> ShapeIdSet {
        self.selected_shapes.clone()
    }
}

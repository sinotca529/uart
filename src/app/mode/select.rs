use std::collections::HashMap;

use super::{Mode, NextMode};
use crate::{
    app::{
        canvas::{CanvasHandler, ShapeId, ShapeIdSet},
        keybind::KeySequence,
        keybind_manager::EventToOp,
        AppOp,
    },
    util::Direction,
};
use crossterm::event::Event;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub enum Op {
    ToggleSelect,
    MoveCursor(Direction),
    MoveShapes(Direction),
    DeleteShapes,
    EnterNormalMode,
    Nop,
}

pub struct SelectMode<'a> {
    selected_shapes: ShapeIdSet,
    kb_mgr: EventToOp<'a, Op>,
}

impl<'a> SelectMode<'a> {
    /// id: initial selected shape
    pub fn new(keybindings: &'a HashMap<KeySequence, Op>, id: ShapeId) -> Self {
        let mut selected_shapes = ShapeIdSet::default();
        selected_shapes.insert(&id);
        Self {
            selected_shapes,
            kb_mgr: EventToOp::new(keybindings),
        }
    }
}

impl Mode for SelectMode<'_> {
    fn next(&mut self, e: Event, canvas_handler: &CanvasHandler) -> (NextMode, crate::app::AppOp) {
        let Some(op) = self.kb_mgr.convert(e) else {
            return (NextMode::Current, AppOp::Nop);
        };

        match op {
            Op::ToggleSelect => {
                let Some(id) = canvas_handler.shape_id_under_the_cursor() else {
                    return (NextMode::Current, AppOp::Nop);
                };
                self.selected_shapes.toggle(&id);
                if self.selected_shapes.is_empty() {
                    (NextMode::Normal, AppOp::Nop)
                } else {
                    (NextMode::Current, AppOp::Nop)
                }
            }
            Op::MoveCursor(d) => (NextMode::Current, AppOp::MoveCanvasCursor(d)),
            Op::MoveShapes(d) => {
                let shapes = self.selected_shapes.clone();
                (
                    NextMode::Current,
                    AppOp::MoveShapesAndCanvasCursor(shapes, d),
                )
            }
            Op::DeleteShapes => {
                let shapes = std::mem::take(&mut self.selected_shapes);
                (NextMode::Normal, AppOp::DeleteShapes(shapes))
            }
            Op::EnterNormalMode => (NextMode::Normal, AppOp::Nop),
            Op::Nop => (NextMode::Current, AppOp::Nop),
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

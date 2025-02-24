use std::collections::HashMap;

use super::{Mode, NextMode};
use crate::{
    app::{canvas::CanvasHandler, keybind::KeySequence, keybind_manager::EventToOp, AppOp},
    util::Direction,
};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;

/// Operations for normal mode.
#[derive(Clone, Deserialize, Debug)]
pub enum Op {
    /// Change to cmd mode.
    #[serde(skip)]
    EnterCmd,
    /// Change to make rect mode.
    EnterMakeRect,
    /// Change to make line mode.
    EnterMakePath,
    /// Change to make text mode.
    EnterMakeText,
    /// Move Cursor
    MoveCursor(Direction),
    /// Toggle the selection state of the shape directly under the cursor.
    EnterSelectShape,
}

pub struct NormalMode<'a> {
    kb_mgr: EventToOp<'a, Op>,
}

impl<'a> NormalMode<'a> {
    pub fn new(keybindings: &'a HashMap<KeySequence, Op>) -> Self {
        Self {
            kb_mgr: EventToOp::new(keybindings),
        }
    }
}

impl Mode for NormalMode<'_> {
    fn next(&mut self, e: Event, canvas_handler: &CanvasHandler) -> (NextMode, AppOp) {
        let force_op = match e {
            Event::Key(k) => match k.code {
                KeyCode::Char(':') => Some(Op::EnterCmd),
                _ => None,
            },
            _ => None,
        };
        let Some(op) = force_op.or(self.kb_mgr.convert(e)) else {
            return (NextMode::Current, AppOp::Nop);
        };

        match op {
            Op::EnterCmd => (NextMode::Command, AppOp::Nop),
            Op::MoveCursor(d) => (NextMode::Current, AppOp::MoveCanvasCursor(d)),
            Op::EnterMakeRect => (NextMode::MakeRect, AppOp::Nop),
            Op::EnterMakePath => (NextMode::MakePath, AppOp::Nop),
            Op::EnterMakeText => (NextMode::MakeText, AppOp::Nop),
            Op::EnterSelectShape => match canvas_handler.shape_id_under_the_cursor() {
                Some(id) => (NextMode::Select(id), AppOp::Nop),
                None => (NextMode::Current, AppOp::Nop),
            },
        }
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw(
            "NORM [:]cmd [r]rect [t]text [p]path [ ]select [hjkl]move cursor",
        );
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

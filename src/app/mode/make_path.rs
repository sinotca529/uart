use std::collections::HashMap;

use crate::{
    app::{
        canvas::CanvasHandler,
        keybind::KeySequence,
        keybind_manager::EventToOp,
        shape::{path::Path, style::Style, Shape},
        AppOp,
    },
    util::{Coord, Direction},
};
use crossterm::event::Event;
use ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;

use super::{Mode, NextMode};

#[derive(Clone, Deserialize, Debug)]
pub enum Op {
    MoveCursor(Direction),
    /// Pop one step from path
    Back,
    MakeShape,
    SelectNextStyle,
    SelectNextArrowState,
    Nop,
}

pub struct MakePathMode<'a> {
    start_coord: Coord,
    path: Path,
    kb_mgr: EventToOp<'a, Op>,
}

impl<'a> MakePathMode<'a> {
    pub fn new(keybindings: &'a HashMap<KeySequence, Op>, canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            path: Path::new(vec![], false, false, Style::Single),
            kb_mgr: EventToOp::new(keybindings),
        }
    }
}

impl Mode for MakePathMode<'_> {
    fn next(&mut self, e: Event, _canvas_handler: &CanvasHandler) -> (NextMode, crate::app::AppOp) {
        let Some(op) = self.kb_mgr.convert(e) else {
            return (NextMode::Current, AppOp::Nop);
        };

        match op {
            Op::Nop => (NextMode::Current, AppOp::Nop),
            Op::MoveCursor(d) => {
                self.path.push_path(d);
                (NextMode::Current, AppOp::MoveCanvasCursor(d))
            }
            Op::Back => match self.path.pop_path() {
                Some(dir) => (NextMode::Current, AppOp::MoveCanvasCursor(dir.opposite())),
                None => (NextMode::Current, AppOp::Nop),
            },
            Op::MakeShape => {
                let op = if self.path.is_empty() {
                    AppOp::Nop
                } else {
                    let upper_left = self.start_coord + self.path.start_to_upper_left();
                    AppOp::MakeShape(upper_left, Box::new(self.path.clone()))
                };
                (NextMode::Normal, op)
            }
            Op::SelectNextStyle => {
                self.path.set_next_line_style();
                (NextMode::Current, AppOp::Nop)
            }
            Op::SelectNextArrowState => {
                self.path.set_next_arrow_state();
                (NextMode::Current, AppOp::Nop)
            }
        }
    }

    fn additinal_canvas_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let start = self.start_coord + self.path.start_to_upper_left();
        vec![(start, Box::new(self.path.clone()))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw(
            "LINE [↵]Complete, [s]Change Line Style [a]Change Arrow State",
        );
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

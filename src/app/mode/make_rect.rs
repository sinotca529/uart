use super::{Mode, NextMode};
use crate::{
    app::{
        canvas::CanvasHandler,
        keybind::KeySequence,
        keybind_manager::EventToOp,
        shape::{rect::Rect, style::Style, Shape},
        AppOp,
    },
    util::{Coord, Direction, Size},
};
use crossterm::event::Event;
use ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Debug)]
pub enum Op {
    MoveCursor(Direction),
    MakeShape,
    NextStyle,
}

pub struct MakeRectMode<'a> {
    start_coord: Coord,
    rect: Rect,
    kb_mgr: EventToOp<'a, Op>,
}

impl<'a> MakeRectMode<'a> {
    pub fn new(keybindings: &'a HashMap<KeySequence, Op>, canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            rect: Rect::new(Size::new(1, 1), Style::Single),
            kb_mgr: EventToOp::new(keybindings),
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
        let (w, h) = (diff.x.unsigned_abs() + 1, diff.y.unsigned_abs() + 1);
        self.rect.set_size(Size::new(w, h));
    }
}

impl Mode for MakeRectMode<'_> {
    fn next(&mut self, e: Event, canvas_handler: &CanvasHandler) -> (NextMode, AppOp) {
        let Some(op) = self.kb_mgr.convert(e) else {
            return (NextMode::Current, AppOp::Nop);
        };

        match op {
            Op::MoveCursor(d) => {
                self.update_rect_size(canvas_handler.cursor_coord().adjacency(d));
                (NextMode::Current, AppOp::MoveCanvasCursor(d))
            }
            Op::NextStyle => {
                self.rect.set_next_line_style();
                (NextMode::Current, AppOp::Nop)
            }
            Op::MakeShape => {
                let upper_left = self.upper_left_corner(canvas_handler.cursor_coord());
                let op = AppOp::MakeShape(upper_left, Box::new(self.rect));
                (NextMode::Normal, op)
            }
        }
    }

    fn additinal_canvas_shapes(&self, canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let upper_left = self.upper_left_corner(canvas_cursor);
        vec![(upper_left, Box::new(self.rect))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("RECT [↵]Complete, [s]Change Line Style");
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

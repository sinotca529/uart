use std::collections::HashMap;

use super::{Mode, NextMode};
use crate::{
    app::{
        canvas::CanvasHandler,
        keybind::KeySequence,
        keybind_manager::EventToOp,
        shape::{text::Text, Shape},
        AppOp,
    },
    util::Coord,
};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;
use unicode_width::UnicodeWidthChar;

#[derive(Clone, Deserialize, Debug)]
pub enum Op {
    MakeText,
    AddChar(char),
    Enter,
    Backspace,
}

pub struct MakeTextMode<'a> {
    start_coord: Coord,
    text: Text,
    kb_mgr: EventToOp<'a, Op>,
}

impl<'a> MakeTextMode<'a> {
    pub fn new(keybindings: &'a HashMap<KeySequence, Op>, canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            text: Text::new(String::new()),
            kb_mgr: EventToOp::new(keybindings),
        }
    }
}

impl Mode for MakeTextMode<'_> {
    fn next(&mut self, e: Event, canvas_handler: &CanvasHandler) -> (NextMode, AppOp) {
        let fall_back_op = match e {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => Some(Op::AddChar(c)),
                KeyCode::Backspace => Some(Op::Backspace),
                KeyCode::Enter => Some(Op::Enter),
                _ => None,
            },
            _ => None,
        };
        let Some(op) = self.kb_mgr.convert(e).or(fall_back_op) else {
            return (NextMode::Current, AppOp::Nop);
        };

        let mut cursor_coord = canvas_handler.cursor_coord();
        match op {
            Op::MakeText => {
                let op = if self.text.is_empty() {
                    AppOp::Nop
                } else {
                    let text = std::mem::take(&mut self.text);
                    AppOp::MakeShape(self.start_coord, Box::new(text))
                };
                (NextMode::Normal, op)
            }
            Op::AddChar(c) => {
                self.text.push(c);
                cursor_coord.x += c.width().unwrap() as i16;
                (NextMode::Current, AppOp::SetCanvasCursor(cursor_coord))
            }
            Op::Enter => {
                self.text.push('\n');
                cursor_coord.y += 1;
                cursor_coord.x = self.start_coord.x;
                (NextMode::Current, AppOp::SetCanvasCursor(cursor_coord))
            }
            Op::Backspace => {
                let c = self.text.pop();
                match c {
                    Some('\n') => {
                        cursor_coord.y -= 1;
                        cursor_coord.x += self.text.last_line_width().unwrap_or(0) as i16;
                    }
                    Some(c) => {
                        cursor_coord.x -= c.width().unwrap() as i16;
                    }
                    _ => {}
                }
                (NextMode::Current, AppOp::SetCanvasCursor(cursor_coord))
            }
        }
    }

    fn additinal_canvas_shapes(&self, _: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        vec![(self.start_coord, Box::new(self.text.clone()))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("TEXT [S-CR]Complete");
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

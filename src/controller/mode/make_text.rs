use crossterm::event::{Event, KeyCode};
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
};

use crate::{
    canvas::shape::{text::Text, Shape},
    controller::AppOp,
    util::Coord,
};

use super::{normal::NormalMode, Mode};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

enum Op {
    MakeText,
    AddChar(char),
    Enter,
    Backspace,
    Nop,
}

impl From<Event> for Op {
    fn from(e: Event) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Enter => Op::Enter,
                KeyCode::Char(c) => Op::AddChar(c),
                KeyCode::Backspace => Op::Backspace,
                KeyCode::Esc => Op::MakeText,
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct MakeTextMode {
    start_coord: Coord,
    text: String,
}

impl MakeTextMode {
    pub fn new(canvas_cursor: Coord) -> Self {
        Self {
            start_coord: canvas_cursor,
            text: String::new(),
        }
    }
}

impl Mode for MakeTextMode {
    fn next(mut self: Box<Self>, e: Event, mut canvas_cursor: Coord) -> (Box<dyn Mode>, AppOp) {
        match e.into() {
            Op::Nop => (self, AppOp::Nop),
            Op::MakeText => {
                let mode = Box::new(NormalMode);
                let text = Box::new(Text::new(self.text.clone()));
                let op = AppOp::MakeShape(self.start_coord, text);
                (mode, op)
            }
            Op::AddChar(c) => {
                self.text.push(c);
                canvas_cursor.x += UnicodeWidthChar::width(c).unwrap() as u16;
                (self, AppOp::SetCanvasCursor(canvas_cursor))
            }
            Op::Enter => {
                self.text.push('\n');
                canvas_cursor.y += 1;
                canvas_cursor.x = self.start_coord.x;
                (self, AppOp::SetCanvasCursor(canvas_cursor))
            }
            Op::Backspace => {
                let c = self.text.pop();
                match c {
                    Some('\n') => {
                        canvas_cursor.y -= 1;
                        canvas_cursor.x +=
                            UnicodeWidthStr::width(self.text.lines().last().unwrap_or("")) as u16;
                    }
                    Some(c) => {
                        canvas_cursor.x -= UnicodeWidthChar::width(c).unwrap() as u16;
                    }
                    _ => {}
                }
                (self, AppOp::SetCanvasCursor(canvas_cursor))
            }
        }
    }

    fn additional_shapes(&self, _: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let text = Text::new(self.text.clone());
        vec![(self.start_coord, Box::new(text))]
    }

    fn status_msg(&self) -> tui::widgets::Paragraph {
        let t = tui::text::Text::raw("TEXT [Esc]Complete");
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

use super::{normal::NormalMode, Mode};
use crate::{
    app::{
        canvas::CanvasHandler,
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
    fn next(
        mut self: Box<Self>,
        e: Event,
        canvas_handler: &CanvasHandler,
    ) -> (Box<dyn Mode>, AppOp) {
        let mut cursor_coord = canvas_handler.cursor_coord();
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
                cursor_coord.x += UnicodeWidthChar::width(c).unwrap() as i16;
                (self, AppOp::SetCanvasCursor(cursor_coord))
            }
            Op::Enter => {
                self.text.push('\n');
                cursor_coord.y += 1;
                cursor_coord.x = self.start_coord.x;
                (self, AppOp::SetCanvasCursor(cursor_coord))
            }
            Op::Backspace => {
                let c = self.text.pop();
                match c {
                    Some('\n') => {
                        let last_line = self.text.lines().last().unwrap_or("");
                        let last_line_width = UnicodeWidthStr::width(last_line) as i16;
                        cursor_coord.y -= 1;
                        cursor_coord.x += last_line_width;
                    }
                    Some(c) => {
                        cursor_coord.x -= UnicodeWidthChar::width(c).unwrap() as i16;
                    }
                    _ => {}
                }
                (self, AppOp::SetCanvasCursor(cursor_coord))
            }
        }
    }

    fn additinal_canvas_shapes(&self, _: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        let text = Text::new(self.text.clone());
        vec![(self.start_coord, Box::new(text))]
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("TEXT [Esc]Complete");
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

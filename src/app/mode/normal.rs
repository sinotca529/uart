use super::{
    command::CmdMode, make_rect::MakeRectMode, make_text::MakeTextMode, select::SelectMode, Mode,
};
use crate::{
    app::{
        canvas::{CanvasHandler, ShapeId},
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

/// Operations for normal mode.
enum Op {
    /// Change to cmd mode.
    EnterCmd,
    /// Change to make rect mode.
    EnterMakeRect,
    /// Change to make text mode.
    EnterMakeText,
    /// Move Cursor
    MoveCursor(Direction),
    /// Toggle the selection state of the shape directly under the cursor.
    EnterSelectShape(ShapeId),
    /// Do nothing.
    Nop,
}

impl From<(Event, &CanvasHandler)> for Op {
    fn from((e, ch): (Event, &CanvasHandler)) -> Self {
        match e {
            Event::Key(k) => match k.code {
                KeyCode::Char(c) => match c {
                    ':' => Op::EnterCmd,
                    'h' => Op::MoveCursor(Direction::Left),
                    'j' => Op::MoveCursor(Direction::Down),
                    'k' => Op::MoveCursor(Direction::Up),
                    'l' => Op::MoveCursor(Direction::Right),
                    'r' => Op::EnterMakeRect,
                    't' => Op::EnterMakeText,
                    ' ' => match ch.shape_id_under_the_cursor() {
                        Some(id) => Op::EnterSelectShape(id),
                        None => Op::Nop,
                    },
                    _ => Op::Nop,
                },
                _ => Op::Nop,
            },
            _ => Op::Nop,
        }
    }
}

pub struct NormalMode;

impl NormalMode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NormalMode {
    fn default() -> Self {
        Self::new()
    }
}

impl Mode for NormalMode {
    fn next(self: Box<Self>, e: Event, canvas_handler: &CanvasHandler) -> (Box<dyn Mode>, AppOp) {
        let cursor = canvas_handler.cursor();
        match (e, canvas_handler).into() {
            Op::EnterCmd => {
                let cmd = Box::new(CmdMode::new());
                (cmd, AppOp::Nop)
            }
            Op::Nop => (self, AppOp::Nop),
            Op::MoveCursor(d) => (self, AppOp::MoveCanvasCursor(d)),
            Op::EnterMakeRect => (Box::new(MakeRectMode::new(cursor.coord())), AppOp::Nop),
            Op::EnterMakeText => (Box::new(MakeTextMode::new(cursor.coord())), AppOp::Nop),
            Op::EnterSelectShape(id) => (Box::new(SelectMode::new(id)), AppOp::Nop),
        }
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        let t = ratatui::text::Text::raw("NORM [:]cmd [r]rect [t]text");
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

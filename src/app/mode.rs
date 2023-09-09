mod command;
mod make_rect;
mod make_text;
mod normal;

use self::normal::NormalMode;
use super::{canvas::cursor::Cursor, cmd_line::CmdLine, shape::Shape, AppOp};
use crate::util::UCoord;
use crossterm::event::Event;
use tui::widgets::Paragraph;

pub trait Mode {
    fn next(self: Box<Self>, e: Event, cursor: &Cursor) -> (Box<dyn Mode>, AppOp);

    /// Additional shapes to render on the canvas.
    fn additinal_canvas_shapes(&self, _canvas_cursor: UCoord) -> Vec<(UCoord, Box<dyn Shape>)> {
        vec![]
    }

    /// Message to show in command line.
    fn status_msg(&self) -> Paragraph;

    fn cmd_line(&self) -> CmdLine {
        CmdLine::new(self.status_msg())
    }
}

pub struct ModeHandler(Box<dyn Mode>);

impl Default for ModeHandler {
    fn default() -> Self {
        Self(Box::new(NormalMode::new()))
    }
}

impl ModeHandler {
    pub fn process_event(&mut self, event: Event, cursor: &Cursor) -> AppOp {
        unsafe {
            let current_mode: Box<dyn Mode> = std::ptr::read(&self.0);
            let (next_mode, app_op) = current_mode.next(event, cursor);
            std::ptr::write(&mut self.0, next_mode);
            app_op
        }
    }

    pub fn get(&self) -> &dyn Mode {
        self.0.as_ref()
    }
}

mod command;
mod dummy;
mod make_rect;
mod make_text;
mod normal;
mod select;

use self::normal::NormalMode;
use super::{
    canvas::{CanvasHandler, ShapeIdSet},
    cmd_line::CmdLine,
    shape::Shape,
    AppOp,
};
use crate::util::Coord;
use crossterm::event::Event;
use dummy::DummyMode;
use ratatui::widgets::Paragraph;

pub trait Mode {
    fn next(self: Box<Self>, e: Event, canvas_handler: &CanvasHandler) -> (Box<dyn Mode>, AppOp);

    /// Additional shapes to render on the canvas.
    fn additinal_canvas_shapes(&self, _canvas_cursor: Coord) -> Vec<(Coord, Box<dyn Shape>)> {
        vec![]
    }

    /// Shapes to highlight
    fn shapes_to_highlight(&self) -> ShapeIdSet {
        Default::default()
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
    pub fn process_event(&mut self, event: Event, canvas_handler: &CanvasHandler) -> AppOp {
        let current_mode = std::mem::replace(&mut self.0, Box::new(DummyMode::new()));
        let (next_mode, app_op) = current_mode.next(event, canvas_handler);
        self.0 = next_mode;
        app_op
    }

    pub fn get(&self) -> &dyn Mode {
        self.0.as_ref()
    }
}

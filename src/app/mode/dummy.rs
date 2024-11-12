use super::Mode;
use crate::app::{canvas::CanvasHandler, AppOp};
use crossterm::event::Event;

pub struct DummyMode;

impl DummyMode {
    pub fn new() -> Self {
        Self
    }
}

impl Mode for DummyMode {
    fn next(self: Box<Self>, _e: Event, _canvas_handler: &CanvasHandler) -> (Box<dyn Mode>, AppOp) {
        unreachable!();
    }

    fn status_msg(&self) -> ratatui::widgets::Paragraph {
        unreachable!();
    }
}

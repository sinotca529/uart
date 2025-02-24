mod command;
pub mod make_path;
pub mod make_rect;
pub mod make_text;
pub mod normal;
pub mod select;

use self::normal::NormalMode;
use super::{
    canvas::{CanvasHandler, ShapeId, ShapeIdSet},
    cmd_line::CmdLine,
    config::KeyBindings,
    shape::Shape,
    AppOp,
};
use crate::util::Coord;
use command::CmdMode;
use crossterm::event::Event;
use make_path::MakePathMode;
use make_rect::MakeRectMode;
use make_text::MakeTextMode;
use ratatui::widgets::Paragraph;
use select::SelectMode;

pub trait Mode {
    fn next(&mut self, e: Event, canvas_handler: &CanvasHandler) -> (NextMode, AppOp);

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

pub enum NextMode {
    /// Do not change the mode
    Current,
    Normal,
    Select(ShapeId),
    Command,
    MakeRect,
    MakeText,
    MakePath,
}

pub struct ModeHandler<'a> {
    mode: Box<dyn Mode + 'a>,
    key_bindings: &'a KeyBindings,
}

impl<'a> ModeHandler<'a> {
    pub fn new(key_bindings: &'a KeyBindings) -> Self {
        Self {
            mode: Box::new(NormalMode::new(&key_bindings.normal)),
            key_bindings,
        }
    }

    pub fn process_event(&mut self, event: Event, canvas_handler: &CanvasHandler) -> AppOp {
        let cursor = canvas_handler.cursor_coord();
        let (next_mode, app_op) = self.mode.next(event, canvas_handler);
        match next_mode {
            NextMode::Current => {}
            NextMode::Normal => self.mode = Box::new(NormalMode::new(&self.key_bindings.normal)),
            NextMode::Select(id) => {
                self.mode = Box::new(SelectMode::new(&self.key_bindings.select, id))
            }
            NextMode::Command => self.mode = Box::new(CmdMode::new()),
            NextMode::MakeRect => {
                self.mode = Box::new(MakeRectMode::new(&self.key_bindings.make_rect, cursor))
            }
            NextMode::MakeText => {
                self.mode = Box::new(MakeTextMode::new(&self.key_bindings.make_text, cursor))
            }
            NextMode::MakePath => {
                self.mode = Box::new(MakePathMode::new(&self.key_bindings.make_path, cursor))
            }
        };
        app_op
    }

    pub fn get(&self) -> &dyn Mode {
        self.mode.as_ref()
    }
}

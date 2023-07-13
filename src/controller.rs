use crate::{
    canvas::shape::Shape,
    util::{UCoord, Direction},
};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeShape(UCoord, Box<dyn Shape>),
    MoveCanvasCursor(Direction),
    SetCanvasCursor(UCoord),
    QuitApp,
    Nop,
}

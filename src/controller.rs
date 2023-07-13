use crate::{
    canvas::shape::Shape,
    util::{Direction, UCoord},
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

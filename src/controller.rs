use crate::{
    canvas::shape::Shape,
    util::{Coord, Direction},
};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeShape(Coord, Box<dyn Shape>),
    MoveCanvasCursor(Direction),
    SetCanvasCursor(Coord),
    QuitApp,
    Nop,
}

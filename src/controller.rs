use crate::{
    canvas::shape::Shape,
    util::{Coord, Direction},
};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeShape(Coord, Shape),
    MoveCanvasCursor(Direction),
    SetCanvasCursor(Coord),
    QuitApp,
    Nop,
}

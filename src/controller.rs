use crate::{canvas::shape::Shape, util::Coord};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeShape(Coord, Shape),
    QuitApp,
    Nop,
}

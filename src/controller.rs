use crate::{canvas::shape::rect::Rect, util::Coord};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeRect(Coord, Rect),
    QuitApp,
    Nop,
}

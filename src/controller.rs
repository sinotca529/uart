use crate::{
    canvas::shape::{rect::Rect, text::Text},
    util::Coord,
};

pub mod mode;

/// Operations for app.
pub enum AppOp {
    MakeRect(Coord, Rect),
    MakeText(Coord, Text),
    QuitApp,
    Nop,
}

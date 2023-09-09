mod coord;
mod direction;
mod id;
mod size;
mod onetime_widget;

pub use coord::*;
pub use direction::*;
pub use id::*;
pub use size::*;
pub use onetime_widget::*;

pub fn make_area(coord: &UCoord, size: &Size) -> tui::layout::Rect {
    tui::layout::Rect::new(coord.x, coord.y, size.width, size.height)
}

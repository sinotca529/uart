mod coord;
mod direction;
mod size;

pub use coord::*;
pub use direction::*;
pub use size::*;

pub fn make_area(coord: &UCoord, size: &Size) -> tui::layout::Rect {
    tui::layout::Rect::new(coord.x, coord.y, size.width, size.height)
}

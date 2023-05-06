use crate::util::{make_area, Coord, Size};
pub mod rect;
pub mod style;
pub mod text;
use rect::Rect;
use tui::{
    layout::Alignment,
    widgets::{Paragraph, Widget},
};

use self::text::Text;

pub trait ShapeIf: ToString {
    fn size(&self) -> Size;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Shape {
    Rect(Rect),
    Text(Text),
}

impl<'a> From<&'a Shape> for &'a dyn ShapeIf {
    fn from(val: &'a Shape) -> Self {
        match val {
            Shape::Rect(i) => i,
            Shape::Text(i) => i,
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        let s: &dyn ShapeIf = self.into();
        s.to_string()
    }
}

pub struct ShapeWithCoord<'a> {
    shape: &'a dyn ShapeIf,
    offset: &'a Coord,
}

impl<'a> ShapeWithCoord<'a> {
    pub fn new(shape: &'a dyn ShapeIf, coord: &'a Coord) -> Self {
        Self {
            shape,
            offset: coord,
        }
    }
}

impl<'a> Widget for ShapeWithCoord<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let upper_left = Coord::new(area.x + self.offset.x, area.y + self.offset.y);
        let shape_area = make_area(&upper_left, &self.shape.size());
        let t: tui::text::Text = self.shape.to_string().into();
        let p = Paragraph::new(t).alignment(Alignment::Left);
        p.render(shape_area, buf);
    }
}

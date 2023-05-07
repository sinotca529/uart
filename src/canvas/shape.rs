use crate::util::{make_area, Coord, InstantWidget, Size};
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

impl Shape {
    /// TODO : move this method to ShapeIf.
    /// (see : https://github.com/rust-lang/rust/issues/91611)
    pub fn renderer(&self, offset: Coord) -> impl Widget + '_ {
        InstantWidget::new(
            move |area: tui::layout::Rect, buf: &mut tui::buffer::Buffer| {
                let s: &dyn ShapeIf = self.into();
                let upper_left = Coord::new(area.x + offset.x, area.y + offset.y);
                let shape_area = make_area(&upper_left, &s.size());
                let t: tui::text::Text = s.to_string().into();
                let p = Paragraph::new(t).alignment(Alignment::Left);
                p.render(shape_area, buf);
            },
        )
    }
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

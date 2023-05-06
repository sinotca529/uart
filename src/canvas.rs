pub mod shape;
mod shape_id;

use self::{
    shape::Shape,
    shape_id::{Id, IdGenerator},
};
use crate::util::{make_area, Coord};
use std::collections::BTreeMap;
use tui::{
    layout::Alignment,
    text::Text,
    widgets::{Paragraph, Widget},
};

pub struct Canvas {
    sig: IdGenerator,
    shapes: BTreeMap<Id, (Coord, Shape)>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: BTreeMap::new(),
        }
    }

    pub fn add_shape(&mut self, coord: Coord, shape: Shape) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        assert_eq!(old, None);
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(Coord, Shape)> {
        self.shapes.iter().map(|e| e.1)
    }
}

impl Widget for &Canvas {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        for (coord, shape) in self.shapes() {
            let upper_left = Coord::new(area.x + coord.x, area.y + coord.y);
            let area = make_area(&upper_left, &shape.size());

            let t: Text = shape.to_string().into();
            let p = Paragraph::new(t).alignment(Alignment::Left);
            p.render(area, buf);
        }
    }
}

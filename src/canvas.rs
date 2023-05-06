pub mod shape;
mod shape_id;

use self::{
    shape::{Shape, ShapeWithCoord},
    shape_id::{Id, IdGenerator},
};
use crate::util::Coord;
use std::collections::BTreeMap;
use tui::widgets::Widget;

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
            ShapeWithCoord::new(shape.into(), coord).render(area, buf);
        }
    }
}

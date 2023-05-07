pub mod shape;
mod shape_id;

use self::{
    shape::{Shape, ShapeWithCoord},
    shape_id::{Id, IdGenerator},
};
use crate::controller::mode::Mode;
use crate::util::{Coord, Direction};
use std::collections::BTreeMap;
use tui::{
    style::Color,
    widgets::{StatefulWidget, Widget},
};

pub struct Canvas {
    sig: IdGenerator,
    shapes: BTreeMap<Id, (Coord, Shape)>,
    cursor: Coord,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: BTreeMap::new(),
            cursor: Coord::new(0, 0),
        }
    }

    pub fn cursor(&self) -> Coord {
        self.cursor
    }

    pub fn move_cursor(&mut self, d: Direction) {
        // TODO : consider about wide width char
        self.cursor = self.cursor.adjacency(d);
    }

    pub fn set_cursor(&mut self, c: Coord) {
        self.cursor = c;
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

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> StatefulWidget for &'a Canvas {
    type State = &'a Mode;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        // Render shapes
        for (coord, shape) in self.shapes() {
            ShapeWithCoord::new(shape.into(), coord).render(area, buf);
        }

        // Render mode specific objects
        state.canvas_modify_widget(self.cursor).render(area, buf);

        // Render cursor
        buf.get_mut(area.x + self.cursor.x, area.y + self.cursor.y)
            .set_bg(Color::Rgb(128, 128, 128));
    }
}

pub mod cursor;
pub mod handler;

pub use handler::*;

use self::cursor::Cursor;
use crate::{
    app::shape::Shape,
    util::{Coord, Direction, Id, IdGenerator},
};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
enum ShapeTag {}
type ShapeId = Id<ShapeTag>;
type ShapeIdGen = IdGenerator<ShapeTag>;

#[derive(Default)]
pub struct Canvas {
    sig: ShapeIdGen,
    shapes: BTreeMap<ShapeId, (Coord, Box<dyn Shape>)>,
    cursor: Cursor,
}

impl Canvas {
    /// Add new shape to canvas.
    /// `coord` is the coord of upper-left corner of the shape.
    pub fn add_shape(&mut self, coord: Coord, shape: Box<dyn Shape>) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        // Ensure there is no shape which has same id.
        assert!(old.is_none());
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(Coord, Box<dyn Shape>)> {
        self.shapes.iter().map(|e| e.1)
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        self.cursor.move_next(dir);
    }

    pub fn set_cursor(&mut self, coord: Coord) {
        self.cursor.move_to(coord);
    }
}

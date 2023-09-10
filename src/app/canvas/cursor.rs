use crate::util::{Coord, Direction};

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Cursor(Coord);

impl Cursor {
    pub fn new(x: i16, y: i16) -> Self {
        Self(Coord::new(x, y))
    }

    pub fn move_next(&mut self, d: Direction) {
        self.0 = self.0.adjacency(d);
    }

    pub fn coord(&self) -> Coord {
        self.0
    }

    pub fn x(&self) -> i16 {
        self.0.x
    }

    pub fn y(&self) -> i16 {
        self.0.y
    }

    pub fn move_to(&mut self, coord: Coord) {
        self.0 = coord;
    }
}

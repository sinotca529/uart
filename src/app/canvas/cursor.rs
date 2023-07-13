use crate::util::{Direction, UCoord};

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Cursor(UCoord);

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self(UCoord::new(x, y))
    }

    pub fn move_next(&mut self, d: Direction) {
        self.0 = self.0.adjacency(d);
    }

    pub fn coord(&self) -> UCoord {
        self.0
    }

    pub fn x(&self) -> u16 {
        self.0.x
    }

    pub fn y(&self) -> u16 {
        self.0.y
    }

    pub fn move_to(&mut self, coord: UCoord) {
        self.0 = coord;
    }
}

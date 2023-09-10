use super::Direction;

/// Signed 2-dim coord.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn adjacency(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y.saturating_sub(1)),
            Direction::Left => Self::new(self.x.saturating_sub(1), self.y),
            Direction::Right => Self::new(self.x.saturating_add(1), self.y),
            Direction::Down => Self::new(self.x, self.y.saturating_add(1)),
        }
    }

    pub fn offset(&self, base: Coord) -> Coord {
        Coord::new(self.x - base.x, self.y - base.y)
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

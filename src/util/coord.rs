use super::Direction;
use std::ops::Sub;

/// Unsigned 2-dim coord.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct UCoord {
    pub x: u16,
    pub y: u16,
}

impl UCoord {
    pub fn new(x: u16, y: u16) -> Self {
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

    pub fn offset(&self, base: UCoord) -> ICoord {
        ICoord::new(self.x as i16 - base.x as i16, self.y as i16 - base.y as i16)
    }
}

impl Default for UCoord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Sub for UCoord {
    type Output = UCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/// Signed 2-dim coord.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct ICoord {
    pub x: i16,
    pub y: i16,
}

impl ICoord {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Default for ICoord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl From<UCoord> for ICoord {
    fn from(uc: UCoord) -> Self {
        ICoord::new(uc.x as i16, uc.y as i16)
    }
}

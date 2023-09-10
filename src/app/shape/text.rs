use super::Shape;
use crate::util::{Coord, Size};
use unicode_width::UnicodeWidthStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Text {
    s: String,
}

impl Text {
    pub fn new(s: String) -> Self {
        Self { s }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.s.clone()
    }
}

impl Shape for Text {
    fn size(&self) -> Size {
        let mut h = 0;
        let mut w = 0;

        for l in self.s.lines() {
            h += 1;
            w = w.max(UnicodeWidthStr::width(l) as u16);
        }

        Size::new(w, h)
    }

    fn hit(&self, coord: Coord) -> bool {
        if coord.x < 0 || coord.y < 0 {
            return false;
        }

        if let Some(line_y) = self.s.lines().nth(coord.y as usize) {
            return coord.x < UnicodeWidthStr::width(line_y) as i16;
        }

        false
    }
}

use super::{
    style::{ChipKind::*, Style},
    Shape,
};
use crate::util::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Rect {
    size: Size,
    line_style: Style,
}

impl Rect {
    pub fn new(size: Size, line_style: Style) -> Self {
        Self { size, line_style }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_next_line_style(&mut self) {
        self.line_style = self.line_style.next();
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chips = self.line_style.chips();
        let (width, height) = (self.size.width as usize, self.size.height as usize);
        let mut s = vec![' '; (width + 1) * height];

        // new line & vertical line
        for h in 0..height {
            s[(width + 1) * h] = chips[Vertical];
            s[(width + 1) * h + (width - 1)] = chips[Vertical];
            s[(width + 1) * h + width] = '\n';
        }

        // horizontal line
        for w in 1..(width - 1) {
            s[w] = chips[Horizontal];
            s[w + (width + 1) * (height - 1)] = chips[Horizontal];
        }

        // corner
        s[0] = chips[UpperLeftCorner];
        s[width - 1] = chips[UpperRightCorner];
        s[(width + 1) * (height - 1)] = chips[LowerLeftCorner];
        s[(width + 1) * height - 2] = chips[LowerRightCorner];

        write!(f, "{}", s.into_iter().collect::<String>())
    }
}

impl Shape for Rect {
    fn size(&self) -> Size {
        self.size
    }

    fn fill(&self) -> bool {
        true
    }

    fn hit(&self, coord: Coord) -> bool {
        (0..self.size.width as i16).contains(&coord.x)
            && (0..self.size.height as i16).contains(&coord.y)
    }
}

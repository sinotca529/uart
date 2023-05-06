use super::ShapeIf;
use crate::{model::style::Style, util::*};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Rect {
    size: Size,
    style: Style,
}

impl Rect {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            style: Style::Single,
        }
    }
}

impl ToString for Rect {
    fn to_string(&self) -> String {
        let chips = self.style.chips();
        let (width, height) = (self.size.width as usize, self.size.height as usize);
        let mut s = vec![' '; (width + 1) * height];

        // new line & vertical line
        for h in 0..height {
            s[(width + 1) * h] = chips.vertical;
            s[(width + 1) * h + (width - 1)] = chips.vertical;
            s[(width + 1) * h + width] = '\n';
        }

        // horizontal line
        for w in 1..(width - 1) {
            s[w] = chips.horizontal;
            s[w + (width + 1) * (height - 1)] = chips.horizontal;
        }

        // corner
        s[0] = chips.upper_left_corner;
        s[width - 1] = chips.upper_right_corner;
        s[(width + 1) * (height - 1)] = chips.lower_left_corner;
        s[(width + 1) * height - 2] = chips.lower_right_corner;

        s.into_iter().collect()
    }
}

impl ShapeIf for Rect {
    fn size(&self) -> Size {
        self.size
    }
}

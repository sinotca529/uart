use super::Shape;
use crate::util::Size;
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
}

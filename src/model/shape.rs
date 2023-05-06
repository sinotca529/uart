use crate::util::Size;
pub mod line;
pub mod rect;

use rect::Rect;

pub trait ShapeIf: ToString {
    fn size(&self) -> Size;
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Shape {
    Rect(Rect),
}

impl Shape {
    fn inner(&self) -> &impl ShapeIf {
        match &self {
            Shape::Rect(i) => i,
        }
    }

    pub fn size(&self) -> Size {
        self.inner().size()
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        self.inner().to_string()
    }
}

use crate::util::Size;
pub mod rect;
pub mod style;
pub mod text;

use rect::Rect;

use self::text::Text;

pub trait ShapeIf: ToString {
    fn size(&self) -> Size;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Shape {
    Rect(Rect),
    Text(Text),
}

impl Shape {
    fn inner(&self) -> &dyn ShapeIf {
        match &self {
            Shape::Rect(i) => i,
            Shape::Text(i) => i,
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

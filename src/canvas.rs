pub mod shape;
mod shape_id;

use self::{
    shape::Shape,
    shape_id::{Id, IdGenerator},
};
use crate::util::{make_area, Coord};
use std::collections::HashMap;
use tui::{
    layout::Alignment,
    style::Color,
    text::Text,
    widgets::{Paragraph, Widget},
};

pub struct Canvas {
    sig: IdGenerator,
    shapes: HashMap<Id, (Coord, Shape)>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: HashMap::new(),
        }
    }

    pub fn add_shape(&mut self, coord: Coord, shape: Shape) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        assert_eq!(old, None);
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(Coord, Shape)> {
        self.shapes.values()
    }
}

pub struct CanvasWidget<'a> {
    canvas: &'a Canvas,
    cursor_coord: &'a Coord,
}

impl<'a> CanvasWidget<'a> {
    pub fn new(canvas: &'a Canvas, cursor_coord: &'a Coord) -> Self {
        Self {
            canvas,
            cursor_coord,
        }
    }
}

impl<'a> Widget for CanvasWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        for (coord, shape) in self.canvas.shapes() {
            let upper_left = Coord::new(area.x + coord.x, area.y + coord.y);
            let area = make_area(&upper_left, &shape.size());

            let t: Text = shape.to_string().into();
            let p = Paragraph::new(t).alignment(Alignment::Left);
            p.render(area, buf);
        }
        let Coord { x, y } = &self.cursor_coord;
        buf.get_mut(*x, *y).set_bg(Color::Rgb(128, 128, 128));
    }
}

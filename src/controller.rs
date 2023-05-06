use crate::{
    model::{shape::rect::Rect, Model},
    util::{Coord, Size},
};
pub mod command;
pub mod command_stream;

pub struct Controller {
    model: Model,
}

impl Controller {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn add_rect(&mut self, coord: Coord, size: Size) {
        let rect = crate::model::shape::Shape::Rect(Rect::new(size));
        self.model.add_shape(coord, rect);
    }
}

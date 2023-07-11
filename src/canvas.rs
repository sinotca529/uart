pub mod shape;
mod shape_id;

use self::{
    shape::Shape,
    shape_id::{Id, IdGenerator},
};
use crate::util::{Coord, Direction};
use crate::{controller::mode::Mode, util::Size};
use std::collections::BTreeMap;
use tui::{style::Color, widgets::StatefulWidget};

pub struct Canvas {
    sig: IdGenerator,
    shapes: BTreeMap<Id, (Coord, Box<dyn Shape>)>,
    cursor: Coord,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: BTreeMap::new(),
            cursor: Coord::new(0, 0),
        }
    }

    pub fn cursor(&self) -> Coord {
        self.cursor
    }

    pub fn move_cursor(&mut self, d: Direction) {
        // TODO : consider about wide width char
        self.cursor = self.cursor.adjacency(d);
    }

    pub fn set_cursor(&mut self, c: Coord) {
        self.cursor = c;
    }

    pub fn add_shape(&mut self, coord: Coord, shape: Box<dyn Shape>) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        assert!(old.is_none());
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(Coord, Box<dyn Shape>)> {
        self.shapes.iter().map(|e| e.1)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct CanvasHandler {
    pub canvas: Canvas,
    prev_render_left_top_coord: Coord,
}

pub struct RenderState<'a> {
    mode: &'a Mode,
    canvas_size: Size,
}

impl<'a> RenderState<'a> {
    pub fn new(mode: &'a Mode, canvas_size: Size) -> Self {
        Self { mode, canvas_size }
    }
}

impl<'a> StatefulWidget for &'a mut CanvasHandler {
    type State = RenderState<'a>;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let canvas = &self.canvas;

        // calc offset
        let calc_offset = |c: u16, low: u16, length: u16| -> u16 {
            if c < low {
                c
            } else if c > low + length - 1 {
                c - (length - 1)
            } else {
                low
            }
        };

        let offset: Coord = {
            let x = calc_offset(
                canvas.cursor.x,
                self.prev_render_left_top_coord.x,
                state.canvas_size.width,
            );
            let y = calc_offset(
                canvas.cursor.y,
                self.prev_render_left_top_coord.y,
                state.canvas_size.height,
            );
            Coord::new(x, y)
        };

        // Render shapes
        for (coord, shape) in canvas.shapes() {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render mode specific objects
        for (coord, shape) in state.mode.additional_shapes(canvas.cursor) {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render cursor
        buf.get_mut(
            area.x + canvas.cursor.x - offset.x,
            area.y + canvas.cursor.y - offset.y,
        )
        .set_bg(Color::Rgb(128, 128, 128));

        self.prev_render_left_top_coord = offset;
    }
}

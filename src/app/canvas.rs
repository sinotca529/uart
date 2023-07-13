pub mod cursor;

use self::cursor::Cursor;
use super::{mode::Mode, shape::Shape};
use crate::util::{Id, IdGenerator, Size, UCoord};
use std::collections::BTreeMap;
use tui::{style::Color, widgets::StatefulWidget};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
enum ShapeTag {}
type ShapeId = Id<ShapeTag>;
type ShapeIdGen = IdGenerator<ShapeTag>;

/// Canvas to put shapes.
pub struct Canvas {
    sig: ShapeIdGen,
    shapes: BTreeMap<ShapeId, (UCoord, Box<dyn Shape>)>,
    cursor: Cursor,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: BTreeMap::new(),
            cursor: Cursor::default(),
        }
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Add new shape to canvas.
    /// `coord` is the coord of upper-left corner.
    pub fn add_shape(&mut self, coord: UCoord, shape: Box<dyn Shape>) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        assert!(old.is_none());
    }

    pub fn shapes(&self) -> impl Iterator<Item = &(UCoord, Box<dyn Shape>)> {
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
    prev_render_left_top_coord: UCoord,
}

pub struct RenderState<'a> {
    mode: &'a dyn Mode,
    canvas_size: Size,
}

impl<'a> RenderState<'a> {
    pub fn new(mode: &'a dyn Mode, canvas_size: Size) -> Self {
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

        let offset: UCoord = {
            let x = calc_offset(
                canvas.cursor.x(),
                self.prev_render_left_top_coord.x,
                state.canvas_size.width,
            );
            let y = calc_offset(
                canvas.cursor.y(),
                self.prev_render_left_top_coord.y,
                state.canvas_size.height,
            );
            UCoord::new(x, y)
        };

        // Render shapes (id is used as z-index)
        for (coord, shape) in canvas.shapes() {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render mode specific objects
        for (coord, shape) in state.mode.additional_shapes(canvas.cursor.coord()) {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render cursor
        buf.get_mut(
            area.x + canvas.cursor.x() - offset.x,
            area.y + canvas.cursor.y() - offset.y,
        )
        .set_bg(Color::Rgb(128, 128, 128));

        self.prev_render_left_top_coord = offset;
    }
}

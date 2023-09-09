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
    /// `coord` is the coord of upper-left corner of the shape.
    pub fn add_shape(&mut self, coord: UCoord, shape: Box<dyn Shape>) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        // Ensure there is no shape which has same id.
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
    canvas: Canvas,
    prev_offset: UCoord,
}

impl CanvasHandler {
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn calc_rendering_offset(&self, canvas_size: Size) -> UCoord {
        //
        //       Canvas
        //      ┌─────────────────────────┐
        //      │  Rendering area         │
        //      │  ┌───────────────────┐  │
        //      │  │                   │  │
        //      │  └───────────────────┘  │
        //      └─────────────────────────┘
        // --------+-------------------+-------> x
        //         P                  P+W
        //   (Prev offset)
        //
        //
        //  Cursor Pos Range  | Next offset
        //  ==================|==============
        //    [0, P)          |     C
        //    [P, P+W]        |     P
        //    [P+W, ∞)        |     C - (W - 1)
        //
        let calc = |c: u16, p: u16, w: u16| -> u16 {
            if c < p {
                c
            } else if c < p + w {
                p
            } else {
                c - (w - 1)
            }
        };

        let cursor = &self.canvas.cursor;
        UCoord {
            x: calc(cursor.x(), self.prev_offset.x, canvas_size.width),
            y: calc(cursor.y(), self.prev_offset.y, canvas_size.height),
        }
    }
}

// State used to rendering the canvas.
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
        let offset = self.calc_rendering_offset(state.canvas_size);

        // Render shapes (id is used as z-index)
        for (coord, shape) in canvas.shapes() {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render mode specific objects (TODO : This is not a task of CnavasHandler. move to Mode.)
        for (coord, shape) in state.mode.additional_shapes(canvas.cursor.coord()) {
            shape.render(coord.offset(offset), area, buf);
        }

        // Render cursor (TODO : separate from canvas's rendering)
        buf.get_mut(
            area.x + canvas.cursor.x() - offset.x,
            area.y + canvas.cursor.y() - offset.y,
        )
        .set_bg(Color::Rgb(128, 128, 128));

        self.prev_offset = offset;
    }
}

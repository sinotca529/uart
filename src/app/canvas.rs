pub mod cursor;

use self::cursor::Cursor;
use super::shape::Shape;
use crate::util::{Coord, Direction, Id, IdGenerator, Size};
use std::collections::BTreeMap;
use tui::{style::Color, widgets::StatefulWidget};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
enum ShapeTag {}
type ShapeId = Id<ShapeTag>;
type ShapeIdGen = IdGenerator<ShapeTag>;

/// Canvas to put shapes.
pub struct Canvas {
    sig: ShapeIdGen,
    shapes: BTreeMap<ShapeId, (Coord, Box<dyn Shape>)>,
    rendering_offset: Coord,
    cursor: Cursor,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            sig: IdGenerator::new(),
            shapes: BTreeMap::new(),
            rendering_offset: Coord::default(),
            cursor: Cursor::default(),
        }
    }

    /// Add new shape to canvas.
    /// `coord` is the coord of upper-left corner of the shape.
    pub fn add_shape(&mut self, coord: Coord, shape: Box<dyn Shape>) {
        let id = self.sig.gen();
        let old = self.shapes.insert(id, (coord, shape));
        // Ensure there is no shape which has same id.
        assert!(old.is_none());
    }

    fn shapes(&self) -> impl Iterator<Item = &(Coord, Box<dyn Shape>)> {
        self.shapes.iter().map(|e| e.1)
    }

    /// Update rendering offset.
    /// This method must be called before rendering canvas.
    fn update_rendering_offset(&mut self, canvas_size: Size) {
        //
        //     Canvas
        //    ┌─────────────────────────┐
        //    │      Rendering area     │
        //    │      ┌───────────────┐  │
        //    │      │               │  │
        //    │      └───────────────┘  │
        //    └─────────────────────────┘
        // ---+------+----------------+-----> x
        //    0      P               P+W
        //      (Prev offset)
        //
        //
        //  Cursor Pos Range  | Next offset
        //  ==================|==============
        //    [0, P)          |     C
        //    [P, P+W]        |     P
        //    [P+W, ∞)        |     C - (W - 1)
        //
        let calc = |c: i16, p: i16, w: i16| -> i16 {
            if c < p {
                c
            } else if c < p + w {
                p
            } else {
                c - (w - 1)
            }
        };

        self.rendering_offset = Coord {
            x: calc(
                self.cursor.coord().x,
                self.rendering_offset.x,
                canvas_size.width as i16,
            ),
            y: calc(
                self.cursor.coord().y,
                self.rendering_offset.y,
                canvas_size.height as i16,
            ),
        };
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        self.cursor.move_next(dir);
    }

    pub fn set_cursor(&mut self, coord: Coord) {
        self.cursor.move_to(coord);
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CanvasRenderingState {
    canvas_size: Size,
    additional_shapes: Vec<(Coord, Box<dyn Shape>)>,
}

impl CanvasRenderingState {
    pub fn new(canvas_size: Size, additional_shapes: Vec<(Coord, Box<dyn Shape>)>) -> Self {
        Self {
            canvas_size,
            additional_shapes,
        }
    }
}

impl StatefulWidget for &mut Canvas {
    type State = CanvasRenderingState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        self.update_rendering_offset(state.canvas_size);

        // Render shapes.
        // id is used as z-index (ref: BTreeMap::iter)
        for (coord, shape) in self.shapes().chain(state.additional_shapes.iter()) {
            let offset_from_area = coord.offset(self.rendering_offset);
            shape.render(offset_from_area, area, buf);
        }

        // Render cursor.
        buf.get_mut(
            (area.x as i16 + self.cursor.x() - self.rendering_offset.x) as u16,
            (area.y as i16 + self.cursor.y() - self.rendering_offset.y) as u16,
        )
        .set_bg(Color::Rgb(128, 128, 128));
    }
}

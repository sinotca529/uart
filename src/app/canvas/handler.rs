use super::{cursor::Cursor, Canvas, ShapeId, ShapeTag};
use crate::{
    app::shape::Shape,
    util::{Coord, Direction, IdSet, Size},
};
use ratatui::{style::Color, widgets::Widget};

pub type ShapeIdSet = IdSet<ShapeTag>;

/// Canvas handler
///
/// Responsibility :
/// - Render canvas
#[derive(Default)]
pub struct CanvasHandler {
    canvas: Canvas,
    rendering_offset: Coord,
    rendering_size: Size,
    additional_shapes: Vec<(Coord, Box<dyn Shape>)>,
    shapes_to_highlight: ShapeIdSet,
}

impl CanvasHandler {
    pub fn add_shape(&mut self, coord: Coord, shape: Box<dyn Shape>) {
        self.canvas.add_shape(coord, shape);
    }

    pub fn cursor(&self) -> &Cursor {
        self.canvas.cursor()
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        self.canvas.move_cursor(dir);
    }

    pub fn set_cursor(&mut self, coord: Coord) {
        self.canvas.set_cursor(coord);
    }

    pub fn cursor_coord(&self) -> Coord {
        self.canvas.cursor().coord()
    }
}

// Methods for rendering.
impl CanvasHandler {
    pub fn set_rendering_size(&mut self, size: Size) {
        self.rendering_size = size;
    }

    pub fn set_additional_shapes(&mut self, shapes: Vec<(Coord, Box<dyn Shape>)>) {
        self.additional_shapes = shapes;
    }

    pub fn set_shapes_to_highlight(&mut self, shapes: ShapeIdSet) {
        self.shapes_to_highlight = shapes;
    }

    /// Update rendering offset.
    /// This method must be called before rendering canvas.
    fn update_rendering_offset(&mut self) {
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
                self.canvas.cursor().coord().x,
                self.rendering_offset.x,
                self.rendering_size.width as i16,
            ),
            y: calc(
                self.canvas.cursor().coord().y,
                self.rendering_offset.y,
                self.rendering_size.height as i16,
            ),
        };
    }
}

// Methods for select shapes.
impl CanvasHandler {
    /// Id of the shape directly under the cursor.
    pub fn shape_id_under_the_cursor(&self) -> Option<ShapeId> {
        // Use the iterator in reverse order to select the most front figure.
        self.canvas
            .shapes
            .iter()
            .rev()
            .find(|(_, (coord, shape))| {
                let c = self.canvas.cursor.coord().offset(*coord);
                shape.hit(c)
            })
            .map(|(id, _)| *id)
    }

    pub fn cursor_hits_shape(&self) -> bool {
        self.shape_id_under_the_cursor().is_some()
    }

    pub fn move_shapes(&mut self, ids: &ShapeIdSet, dir: Direction) {
        ids.iter().for_each(|id| self.canvas.move_shape(id, dir));
    }

    pub fn delte_shapes(&mut self, ids: &ShapeIdSet) {
        ids.iter().for_each(|i| self.canvas.delete_shape(i));
    }
}

impl Widget for &mut CanvasHandler {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        self.update_rendering_offset();

        // Render shapes.
        // id is used as z-index (ref: BTreeMap::iter)
        for (id, (coord, shape)) in &self.canvas.shapes {
            let offset_from_area = coord.offset(self.rendering_offset);
            let color = if self.shapes_to_highlight.contains(id) {
                Color::Blue
            } else {
                Color::White
            };
            shape.render(offset_from_area, area, buf, color);
        }
        for (coord, shape) in self.additional_shapes.iter() {
            let offset_from_area = coord.offset(self.rendering_offset);
            shape.render(offset_from_area, area, buf, Color::White);
        }

        // Render cursor.
        let cursor = self.canvas.cursor();
        buf.cell_mut((
            (area.x as i16 + cursor.x() - self.rendering_offset.x) as u16,
            (area.y as i16 + cursor.y() - self.rendering_offset.y) as u16,
        ))
        .unwrap()
        .set_bg(Color::DarkGray);
    }
}

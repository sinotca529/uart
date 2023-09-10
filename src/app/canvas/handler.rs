use tui::{style::Color, widgets::Widget};

use super::{cursor::Cursor, Canvas};
use crate::{
    app::shape::Shape,
    util::{Coord, Direction, Size},
};

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

    pub fn set_rendering_size(&mut self, size: Size) {
        self.rendering_size = size;
    }

    pub fn set_additional_shapes(&mut self, shapes: Vec<(Coord, Box<dyn Shape>)>) {
        self.additional_shapes = shapes;
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

impl Widget for &mut CanvasHandler {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        self.update_rendering_offset();

        // Render shapes.
        // id is used as z-index (ref: BTreeMap::iter)
        for (coord, shape) in self.canvas.shapes().chain(self.additional_shapes.iter()) {
            let offset_from_area = coord.offset(self.rendering_offset);
            shape.render(offset_from_area, area, buf);
        }

        // Render cursor.
        let cursor = self.canvas.cursor();
        buf.get_mut(
            (area.x as i16 + cursor.x() - self.rendering_offset.x) as u16,
            (area.y as i16 + cursor.y() - self.rendering_offset.y) as u16,
        )
        .set_bg(Color::Rgb(128, 128, 128));
    }
}

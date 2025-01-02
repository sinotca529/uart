pub mod path;
pub mod rect;
pub mod style;
pub mod text;

use crate::util::{Coord, IterExt, Size, StrExt};
use ratatui::style::{Color, Style};
use std::ops::Range;
use unicode_width::UnicodeWidthChar;

pub trait Shape: ToString {
    fn size(&self) -> Size;

    /// Fill the shape.
    fn fill(&self) -> bool;

    fn render(
        &self,
        offset: Coord,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        color: Color,
    ) {
        //
        // offset > 0
        //
        //    area
        //    ┌────────────────────────┐
        //    │                        │
        //    │<---------- a --------->│
        //    │       <-------- s -----│---->
        //    │<- o ->┌────────────────│─────┐
        //    │       └────────────────│─────┘
        //    │       <- min(a-o, s) ->│
        //    │                        │
        //
        //
        // offset < 0
        //             area
        //             ┌────────────────────────┐
        //             │                        │
        //   <- (-o) ->│<---------- a --------->│
        //   ┌─────────│────────────────────────│─────┐
        //   └─────────│────────────────────────│─────┘
        //   <---------│-------- s -------------│----->
        //   <---------│-- min(a-o, s) -------->│
        //             │                        │
        //
        //
        // => range_to_render = max(0, -o)..min(a-o, s)
        let range_to_render = |offset: i16, area: u16, size: u16| -> Range<usize> {
            let start = 0.max(-offset) as usize;
            let end = (area as i16 - offset).min(size as i16).max(0) as usize;
            start..end
        };
        let x_range = range_to_render(offset.x, area.width, self.size().width);
        let y_range = range_to_render(offset.y, area.height, self.size().height);

        // Skip if the shape is out of the area.
        if x_range.is_empty() || y_range.is_empty() {
            return;
        }

        let cut: String = self
            .to_string()
            .lines()
            .range(&y_range)
            .map(|l| l.slice_by_width(&x_range))
            .collect();

        // Render
        let style = Style::default().fg(color);
        let shape_area = ratatui::layout::Rect::new(
            area.x + 0.max(offset.x) as u16,
            area.y + 0.max(offset.y) as u16,
            x_range.len() as u16,
            y_range.len() as u16,
        );

        let mut x = shape_area.x;
        let mut y = shape_area.y;
        for c in cut.chars() {
            if c == '\n' {
                x = shape_area.x;
                y += 1;
                continue;
            }
            if !c.is_whitespace() || self.fill() {
                buf.set_string(x, y, c.to_string(), style);
            }
            x += UnicodeWidthChar::width(c).unwrap() as u16;
        }
    }

    /// Return true if the coord is on the shape.
    fn hit(&self, coord: Coord) -> bool;
}

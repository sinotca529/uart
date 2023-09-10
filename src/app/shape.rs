pub mod rect;
pub mod style;
pub mod text;

use crate::util::{Coord, Size};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};
use unicode_width::UnicodeWidthChar;

pub trait Shape: ToString {
    fn size(&self) -> Size;

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
        //    │       <-------- w -----│---->
        //    │<- o ->┌────────────────│─────┐
        //    │       └────────────────│─────┘
        //    │       <- min(a-o, w) ->│
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
        //   <---------│-------- w -------------│----->
        //   <---------│-- min(a-o, w) -------->│
        //             │                        │
        //
        //
        // => range_to_render = [max(0, -o), min(a-o, w) - 1]

        let x_range =
            0.max(-offset.x)..(area.width as i16 - offset.x).min(self.size().width as i16);
        let y_range =
            0.max(-offset.y)..(area.height as i16 - offset.y).min(self.size().height as i16);

        // Skip if the shape is out of the area.
        if x_range.is_empty() || y_range.is_empty() {
            return;
        }

        let cut: String = self
            .to_string()
            .lines()
            // cut top/bottom
            .skip(y_range.start as usize)
            .take(y_range.len())
            // cut left/right
            .map(|l| {
                l.chars()
                    .scan(-1, |width, c| {
                        // width : offset of the end of the char (0-origin).
                        //    abあc -> (0, a), (1, b), (3, あ), (4, c)
                        let delta = UnicodeWidthChar::width(c).unwrap() as i16;
                        *width += delta;

                        // Replace a full-width (全角) char at the edge of the screen with a space.
                        if delta == 2 && *width == x_range.start {
                            Some((*width, ' '))
                        } else {
                            Some((*width, c))
                        }
                    })
                    .skip_while(|&(width, _)| width < x_range.start)
                    .take_while(|&(width, _)| width < x_range.end)
                    .map(|(_, c)| c)
                    .chain(['\n'])
                    .collect::<String>()
            })
            .collect();

        // Render
        let shape_area = ratatui::layout::Rect::new(
            area.x + 0.max(offset.x) as u16,
            area.y + 0.max(offset.y) as u16,
            x_range.len() as u16,
            y_range.len() as u16,
        );
        let t: ratatui::text::Text = cut.into();

        let style = Style::default().fg(color);
        let p = Paragraph::new(t).alignment(Alignment::Left).style(style);
        p.render(shape_area, buf);
    }

    /// Return true if the coord is on the shape.
    fn hit(&self, coord: Coord) -> bool;
}

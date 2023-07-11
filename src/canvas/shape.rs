use crate::util::{make_area, Coord, Offset, Size};
pub mod rect;
pub mod style;
pub mod text;
use tui::{
    layout::Alignment,
    widgets::{Paragraph, Widget},
};
use unicode_width::UnicodeWidthChar;

pub trait Shape: ToString {
    fn size(&self) -> Size;

    fn render(&self, mut offset: Offset, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // Skip if nothing to render.
        let shape_size = self.size();
        let buf_width = buf.area().width;
        let buf_height = buf.area().height;

        if shape_size.width as i16 <= -offset.x
            || shape_size.height as i16 <= -offset.y
            || offset.x >= buf_width as i16
            || offset.y >= buf_height as i16
        {
            return;
        }

        // Cut string with considering offset.
        let buf_width = buf.area().width;
        let buf_height = buf.area().height;

        let cut: String = self
            .to_string()
            .lines()
            .skip(0.max(-offset.y) as usize)
            .take(buf_height as usize - offset.y.max(0) as usize)
            .map(|l| {
                let mut displaied_width = 0;
                let mut trimed_str = String::new();

                let mut chars = l.chars();

                // Trim front
                if offset.x < 0 {
                    let mut tmp_off = offset.x;
                    while tmp_off < 0 {
                        let c = chars.next().unwrap();
                        tmp_off += UnicodeWidthChar::width(c).unwrap() as i16;
                    }
                    for _ in 0..tmp_off {
                        trimed_str.push(' ');
                        displaied_width += 1;
                    }
                }

                // Trim back.
                for c in chars {
                    displaied_width += UnicodeWidthChar::width(c).unwrap() as u16;
                    if displaied_width <= buf_width {
                        trimed_str.push(c);
                    } else {
                        break;
                    }
                }
                trimed_str.push('\n');
                trimed_str
            })
            .collect();

        offset.x = 0.max(offset.x);
        offset.y = 0.max(offset.y);

        // Render
        let upper_left = Coord::new(area.x + offset.x as u16, area.y + offset.y as u16);
        let size = Size::new(
            shape_size.width.min(buf_width),
            shape_size.height.min(buf_height),
        );
        let shape_area = make_area(&upper_left, &size);

        let t: tui::text::Text = cut.into();
        let p = Paragraph::new(t).alignment(Alignment::Left);

        p.render(shape_area, buf);
    }
}

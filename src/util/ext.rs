use std::ops::Range;
use unicode_width::UnicodeWidthChar;

pub trait StrExt {
    fn slice_by_width(&self, range: &Range<usize>) -> String;
}

impl<T: AsRef<str>> StrExt for T {
    /// Slice string by width.
    ///
    /// ```rs
    /// assert_eq!("0123456789".slice_by_width(3..5), "34".to_string());
    /// assert_eq!("01あ456789".slice_by_width(3..5), " 4".to_string());
    /// ```
    fn slice_by_width(&self, range: &Range<usize>) -> String {
        self.as_ref()
            .chars()
            .scan(usize::MAX, |width, c| {
                // width : offset of the end of the char (0-origin).
                //    abあc -> (0, a), (1, b), (3, あ), (4, c)
                let delta = UnicodeWidthChar::width(c).unwrap();
                *width = width.wrapping_add(delta);

                // Replace a full-width (全角) char at the edge of the screen with a space.
                if delta == 2 && *width == range.start {
                    Some((*width, ' '))
                } else {
                    Some((*width, c))
                }
            })
            .skip_while(|&(width, _)| width < range.start)
            .take_while(|&(width, _)| width < range.end)
            .map(|(_, c)| c)
            .chain(['\n'])
            .collect::<String>()
    }
}

pub trait IterExt: Iterator {
    fn range(self, range: &Range<usize>) -> std::iter::Take<std::iter::Skip<Self>>
    where
        Self: Sized,
    {
        self.skip(range.start).take(range.len())
    }
}
impl<I: Iterator> IterExt for I {}

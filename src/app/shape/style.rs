// ─ ━ ┄ ┅ ┈ ┉ ╌ ╍ ═
//
// │ ┃ ┆ ┇ ┊ ┋ ╎ ╏ ║
//
// ┌ ┍ ┎ ┏ ╒ ╓ ╔ ╭
//
// ┐ ┑ ┒ ┓ ╕ ╖ ╗ ╮
//
// └ ┕ ┖ ┗ ╘ ╙ ╚ ╰
// ┘ ┙ ┚ ┛ ╛ ╜ ╝ ╯
//
// ├ ┝ ┞ ┟ ┠ ┡ ┢ ┣ ╞ ╟ ╠
//
// ┤ ┥ ┦ ┧ ┨ ┩ ┪ ┫ ╡ ╢ ╣
//
// ┬ ┭ ┮ ┯ ┰ ┱ ┲ ┳ ╤ ╥ ╦
//
// ┴ ┵ ┶ ┷ ┸ ┹ ┺ ┻ ╧ ╨ ╩
//
// ┼ ┽ ┾ ┿ ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊ ╋
//
// ╪ ╫ ╬
//
// ╱ ╲ ╳ ╴ ╵ ╶ ╷ ╸ ╹ ╺ ╻ ╼ ╽ ╾ ╿

use std::ops::Index;

pub enum ChipKind {
    Horizontal,
    Vertical,
    UpperLeftCorner,
    UpperRightCorner,
    LowerLeftCorner,
    LowerRightCorner,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
}

impl ChipKind {
    const NUM_KINDS: usize = 10;
}

#[derive(Copy, Clone)]
pub struct Chips([char; ChipKind::NUM_KINDS]);

impl Chips {
    const fn new(chips: [char; ChipKind::NUM_KINDS]) -> Self {
        Self(chips)
    }
}

impl Index<ChipKind> for Chips {
    type Output = char;

    fn index(&self, kind: ChipKind) -> &Self::Output {
        &self.0[kind as usize]
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Style {
    Single,
    SingleBold,
    Double,
    Dash,
    Dot,
    Ascii,
}

impl Style {
    const NUM_STYLES: u8 = 6;
}

impl Style {
    pub fn chips(&self) -> Chips {
        use Style::*;
        match self {
            Single => Chips::new(['─', '│', '┌', '┐', '└', '┘', '▲', '▼', '◀', '▶']),
            SingleBold => Chips::new(['━', '┃', '┏', '┓', '┗', '┛', '▲', '▼', '◀', '▶']),
            Double => Chips::new(['═', '║', '╔', '╗', '╚', '╝', '▲', '▼', '◀', '▶']),
            Dash => Chips::new(['╌', '╎', '┌', '┐', '└', '┘', '▲', '▼', '◀', '▶']),
            Dot => Chips::new(['.', '.', '.', '.', '.', '.', '^', 'v', '<', '>']),
            Ascii => Chips::new(['-', '|', '+', '+', '+', '+', '^', 'v', '<', '>']),
        }
    }

    /// Get the next style
    pub fn next(&self) -> Self {
        // safe because the number is %ed by the number of styles
        unsafe { std::mem::transmute((*self as u8 + 1) % Style::NUM_STYLES) }
    }
}

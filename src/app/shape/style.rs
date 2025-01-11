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

pub struct Chips {
    pub horizontal: char,
    pub vertical: char,
    pub upper_left_corner: char,
    pub upper_right_corner: char,
    pub lower_left_corner: char,
    pub lower_right_corner: char,
    pub up_arrow: char,
    pub down_arrow: char,
    pub left_arrow: char,
    pub right_arrow: char,
}

impl Chips {
    fn new(
        horizontal: char,
        vertical: char,
        upper_left_corner: char,
        upper_right_corner: char,
        lower_left_corner: char,
        lower_right_corner: char,
        up_arrow: char,
        down_arrow: char,
        left_arrow: char,
        right_arrow: char,
    ) -> Self {
        Self {
            horizontal,
            vertical,
            upper_left_corner,
            upper_right_corner,
            lower_left_corner,
            lower_right_corner,
            up_arrow,
            down_arrow,
            left_arrow,
            right_arrow,
        }
    }
}

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
    pub fn chips(&self) -> Chips {
        use Style::*;
        match self {
            Single => Chips::new('─', '│', '┌', '┐', '└', '┘', '▲', '▼', '◀', '▶'),
            SingleBold => Chips::new('━', '┃', '┏', '┓', '┗', '┛', '▲', '▼', '◀', '▶'),
            Double => Chips::new('═', '║', '╔', '╗', '╚', '╝', '▲', '▼', '◀', '▶'),
            Dash => Chips::new('╌', '╎', '┌', '┐', '└', '┘', '▲', '▼', '◀', '▶'),
            Dot => Chips::new('.', '.', '.', '.', '.', '.', '^', 'v', '<', '>'),
            Ascii => Chips::new('-', '|', '+', '+', '+', '+', '^', 'v', '<', '>'),
        }
    }

    /// Get the next style
    pub fn next(&self) -> Self {
        use Style::*;
        match self {
            Single => SingleBold,
            SingleBold => Double,
            Double => Dash,
            Dash => Dot,
            Dot => Ascii,
            Ascii => Single,
        }
    }
}

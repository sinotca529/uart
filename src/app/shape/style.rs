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
}

impl Chips {
    fn new(
        horizontal: char,
        vertical: char,
        upper_left: char,
        upper_right: char,
        lower_left: char,
        lower_right: char,
    ) -> Self {
        Self {
            horizontal,
            vertical,
            upper_left_corner: upper_left,
            upper_right_corner: upper_right,
            lower_left_corner: lower_left,
            lower_right_corner: lower_right,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Style {
    Single,
    SingleBold,
    Double,
    Dot,
    Ascii,
}

impl Style {
    pub fn chips(&self) -> Chips {
        use Style::*;
        match self {
            Single => Chips::new('─', '│', '┌', '┐', '└', '┘'),
            SingleBold => Chips::new('━', '┃', '┏', '┓', '┗', '┛'),
            Double => Chips::new('═', '║', '╔', '╗', '╚', '╝'),
            Dot => Chips::new('╌', '╎', '┌', '┐', '└', '┘'),
            Ascii => Chips::new('-', '|', '+', '+', '+', '+'),
        }
    }
}

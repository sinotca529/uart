#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn adjacency(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y.saturating_sub(1)),
            Direction::Left => Self::new(self.x.saturating_sub(1), self.y),
            Direction::Right => Self::new(self.x.saturating_add(1), self.y),
            Direction::Down => Self::new(self.x, self.y.saturating_add(1)),
        }
    }

    pub fn offset(&self, base: Coord) -> Offset {
        Offset::new(self.x as i16 - base.x as i16, self.y as i16 - base.y as i16)
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Offset {
    pub x: i16,
    pub y: i16,
}

impl Offset {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Default for Offset {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn make_area(coord: &Coord, size: &Size) -> tui::layout::Rect {
    tui::layout::Rect::new(coord.x, coord.y, size.width, size.height)
}

pub struct InstantWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    renderer: F,
}

impl<F> InstantWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    pub fn new(renderer: F) -> Self {
        Self { renderer }
    }
}

impl<F> tui::widgets::Widget for InstantWidget<F>
where
    F: FnOnce(tui::layout::Rect, &mut tui::buffer::Buffer),
{
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        (self.renderer)(area, buf);
    }
}

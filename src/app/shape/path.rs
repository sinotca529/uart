use super::{
    style::{ChipKind::*, Style},
    Shape,
};
use crate::util::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Path {
    path: Vec<Direction>,
    size: Size,
    start_to_upper_left: Coord,
    line_style: Style,
    has_start_arrow: bool,
    has_end_arrow: bool,
}

impl Path {
    pub fn new(
        path: Vec<Direction>,
        has_start_arrow: bool,
        has_end_arrow: bool,
        line_style: Style,
    ) -> Self {
        let (size, start_to_upper_left) = Self::calc_size_and_start_to_upper_left(&path);
        Self {
            path,
            size,
            start_to_upper_left,
            line_style,
            has_start_arrow,
            has_end_arrow,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn push_path(&mut self, dir: Direction) {
        self.path.push(dir);
        let (size, start_to_upper_left) = Self::calc_size_and_start_to_upper_left(&self.path);
        self.size = size;
        self.start_to_upper_left = start_to_upper_left;
    }

    pub fn pop_path(&mut self) -> Option<Direction> {
        self.path.pop().inspect(|_| {
            let (size, start_to_upper_left) = Self::calc_size_and_start_to_upper_left(&self.path);
            self.size = size;
            self.start_to_upper_left = start_to_upper_left;
        })
    }

    pub fn set_next_line_style(&mut self) {
        self.line_style = self.line_style.next();
    }

    pub fn set_next_arrow_state(&mut self) {
        let (s, e) = match (self.has_start_arrow, self.has_end_arrow) {
            (true, true) => (false, false),
            (true, false) => (true, true),
            (false, true) => (true, false),
            (false, false) => (false, true),
        };
        self.has_start_arrow = s;
        self.has_end_arrow = e;
    }

    pub fn start_to_upper_left(&self) -> Coord {
        self.start_to_upper_left
    }

    fn calc_size_and_start_to_upper_left(path: &[Direction]) -> (Size, Coord) {
        let (mut max_x, mut min_x, mut max_y, mut min_y) = (0i16, 0i16, 0i16, 0i16);
        let (mut current_x, mut current_y) = (0, 0);

        path.iter().for_each(|d| match d {
            Direction::Up => {
                current_y -= 1;
                min_y = min_y.min(current_y);
            }
            Direction::Down => {
                current_y += 1;
                max_y = max_y.max(current_y);
            }
            Direction::Left => {
                current_x -= 1;
                min_x = min_x.min(current_x);
            }
            Direction::Right => {
                current_x += 1;
                max_x = max_x.max(current_x);
            }
        });

        let height = (max_y - min_y + 1) as u16;
        let width = (max_x - min_x + 1) as u16;
        let size = Size::new(width, height);

        let offset_x = if min_x < 0 { min_x } else { 0 };
        let offset_y = if min_y < 0 { min_y } else { 0 };
        let start_to_upper_left = Coord::new(offset_x, offset_y);

        (size, start_to_upper_left)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            return Ok(());
        }

        use Direction::*;
        let chips = self.line_style.chips();

        // Render path
        let mut before = *self.path.first().unwrap();
        let path = self
            .path
            .iter()
            // Add the last direction to render the last chip
            .chain(std::iter::once(self.path.last().unwrap()));

        let mut line = vec![vec![' '; self.size.width as usize]; self.size.height as usize];
        let mut coord = -self.start_to_upper_left;

        for &d in path {
            let chip = match (before, d) {
                (Up, Up) | (Up, Down) | (Down, Up) | (Down, Down) => chips[Vertical],
                (Left, Left) | (Left, Right) | (Right, Left) | (Right, Right) => chips[Horizontal],
                (Up, Left) | (Right, Down) => chips[UpperRightCorner],
                (Up, Right) | (Left, Down) => chips[UpperLeftCorner],
                (Down, Left) | (Right, Up) => chips[LowerRightCorner],
                (Down, Right) | (Left, Up) => chips[LowerLeftCorner],
            };

            line[coord.y as usize][coord.x as usize] = chip;
            coord = coord.adjacency(d);

            before = d;
        }

        // Add arrow
        if self.has_start_arrow {
            let arrow = match self.path.first().unwrap() {
                Up => chips[DownArrow],
                Down => chips[UpArrow],
                Left => chips[RightArrow],
                Right => chips[LeftArrow],
            };
            let start_coord = -self.start_to_upper_left;
            line[start_coord.y as usize][start_coord.x as usize] = arrow;
        }

        if self.has_end_arrow {
            let (arrow, end_coord) = match self.path.last().unwrap() {
                Up => (chips[UpArrow], coord.adjacency(Down)),
                Down => (chips[DownArrow], coord.adjacency(Up)),
                Left => (chips[LeftArrow], coord.adjacency(Right)),
                Right => (chips[RightArrow], coord.adjacency(Left)),
            };
            line[end_coord.y as usize][end_coord.x as usize] = arrow;
        }

        let s: String = line
            .into_iter()
            .map(|l| l.into_iter().collect::<String>())
            .fold(String::new(), |acc, l| acc + &l + "\n");

        write!(f, "{}", s)
    }
}

impl Shape for Path {
    fn size(&self) -> Size {
        self.size
    }

    fn fill(&self) -> bool {
        false
    }

    fn hit(&self, coord: Coord) -> bool {
        let mut current = -self.start_to_upper_left;
        for &d in &self.path {
            if current == coord {
                return true;
            }
            current = current.adjacency(d);
        }
        current == coord
    }
}

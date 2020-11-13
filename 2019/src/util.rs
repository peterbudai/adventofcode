pub type Coord = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> Coord {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn apply(&self, coord: &Coord) -> Coord {
        let delta = self.delta();
        (coord.0 + delta.0, coord.1 + delta.1)
    }

    pub fn turn(&self, clockwise: bool) -> Self {
        match self {
            Direction::Up if clockwise => Direction::Left,
            Direction::Up => Direction::Right,
            Direction::Left if clockwise => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Down if clockwise => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right if clockwise => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

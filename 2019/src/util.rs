pub type Coord = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn delta(&self) -> Coord {
        match self {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    pub fn apply(&self, coord: &Coord) -> Coord {
        let delta = self.delta();
        (coord.0 + delta.0, coord.1 + delta.1)
    }

    pub fn turn(&self, clockwise: bool) -> Self {
        match self {
            Dir::Up if clockwise => Dir::Left,
            Dir::Up => Dir::Right,
            Dir::Left if clockwise => Dir::Down,
            Dir::Left => Dir::Up,
            Dir::Down if clockwise => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right if clockwise => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn direction() {
        let mut c = (-2, 3);
        let mut d = Dir::Up;

        for _ in 0..4 {
            c = d.apply(&c);
            d = d.turn(true);
        }
        assert_eq!(c, (-2, 3));
        assert_eq!(d, Dir::Up);

        for _ in 0..4 {
            c = d.apply(&c);
            d = d.turn(false);
        }
        assert_eq!(c, (-2, 3));
        assert_eq!(d, Dir::Up);
    }
}
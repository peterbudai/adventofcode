pub type Coord = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl Dir {
    pub fn delta(&self) -> Coord {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    pub fn apply(&self, coord: &Coord) -> Coord {
        let delta = self.delta();
        (coord.0 + delta.0, coord.1 + delta.1)
    }

    pub fn apply_mut(&self, coord: &mut Coord) {
        let delta = self.delta();
        coord.0 += delta.0;
        coord.1 += delta.1;
    }

    pub fn turn(&self, clockwise: bool) -> Self {
        match self {
            Dir::Up if clockwise => Dir::Right,
            Dir::Up => Dir::Left,
            Dir::Left if clockwise => Dir::Up,
            Dir::Left => Dir::Down,
            Dir::Down if clockwise => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Right if clockwise => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn iter() -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter().copied()
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
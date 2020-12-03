//! Utility functions and types for repeating elements of the solutions.

use std::str::FromStr;
use std::error::Error;
use std::marker::{Send, Sync};

/// Parse line-separated input into a vector. Can convert into anything that
/// implements the `FromStr` trait.
/// 
/// # Arguments
/// 
/// `input`: Puzzle input string
/// 
/// # Errors
/// 
/// String conversion errors from `T`.
/// 
/// # Examples
/// 
/// ```
/// # use libaoc::util::parse_lines;
/// let input = parse_lines::<u32>("1\n2\n3")?;
/// assert_eq!(input, [1, 2, 3]);
/// # anyhow::Result::<()>::Ok(())
/// ```
pub fn parse_lines<'a, T>(input: &'a str) -> anyhow::Result<Vec<T>> 
    where T: FromStr, <T as FromStr>::Err: 'static + Error + Send + Sync
{
    Ok(input.lines().map(|line| T::from_str(line)).collect::<Result<Vec<_>, _>>()?)
}

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
    fn parse_lines() {
        //assert_eq!(parse_lines::<chr>("a\nb\nc"), vec!['a','b','c']);
    }

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
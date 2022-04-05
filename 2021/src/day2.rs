//! Solution for [Day 2](https://adventofcode.com/2021/day/2) puzzle.

use std::str::FromStr;

use anyhow::{anyhow, Result, Error};
use crate::util::parse_lines;

#[derive(Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cmd = parts.next().ok_or_else(|| anyhow!("Missing command"))?;
        let arg = parts.next().and_then(|s| s.parse::<usize>().ok()).ok_or_else(|| anyhow!("Missing argument"))?;
        match cmd {
            "up" => Ok(Self::Up(arg)),
            "down" => Ok(Self::Down(arg)),
            "forward" => Ok(Self::Forward(arg)),
            _ => Err(anyhow!("Invalid command")),
        }
    }
}

/// Solution for part 1: product of two entries.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
///
pub fn part1(input: &str) -> Result<usize> {
    let (depth, horiz) = parse_lines::<Command>(input)?
    .into_iter()
    .fold((0usize, 0usize),
        |(depth, horiz), cmd|
            match cmd {
                Command::Up(arg) => (depth - arg, horiz),
                Command::Down(arg) => (depth + arg, horiz),
                Command::Forward(arg) => (depth, horiz + arg),
            }
    );
    Ok(depth * horiz)
}

/// Solution for part 2: product of three entries.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
///
pub fn part2(input: &str) -> Result<usize> {
    let (_, depth, horiz) = parse_lines::<Command>(input)?
    .into_iter()
    .fold((0usize, 0usize, 0usize),
        |(aim, depth, horiz), cmd|
            match cmd {
                Command::Up(arg) => (aim - arg, depth, horiz),
                Command::Down(arg) => (aim + arg, depth, horiz),
                Command::Forward(arg) => (aim, depth + (arg * aim), horiz + arg),
            }
    );
    Ok(depth * horiz)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    static INPUT: &str = indoc!(
        "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"
     );

    #[test]
    fn count() -> Result<()> {
        assert_eq!(part1(INPUT)?, 150);
        assert_eq!(part2(INPUT)?, 900);
        Ok(())
    }
}

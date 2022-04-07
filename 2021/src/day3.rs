//! Solution for [Day 2](https://adventofcode.com/2021/day/2) puzzle.

use anyhow::{Result};
use crate::util::{parse_lines};

fn count_ones(lines: &[String]) -> Vec<usize> {
    lines.iter().fold(vec![0usize; lines[0].len()], |mut o, s| {
        for (count, char) in o.iter_mut().zip(s.chars()) {
            if char == '1' {
                *count += 1;
            }
        }
        o
    })
}

fn rates(ones: &[usize], lines: usize) -> (usize, usize) {
    let mut gamma = 0usize;
    for count in ones {
        gamma <<= 1;
        if *count > lines / 2 {
            gamma |= 1;
        }
    }
    let epsilon = !gamma & ((1 << ones.len()) - 1);
    (gamma, epsilon)
}

pub fn part1(input: &str) -> Result<usize> {
    let lines = parse_lines::<String>(input)?;
    let (gamma, epsilon) = rates(&count_ones(&lines), lines.len());
    Ok(gamma * epsilon)
}

pub fn part2(input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    static INPUT: &str = indoc!(
        "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"
     );

    #[test]
    fn solution1() -> Result<()> {
        let lines = parse_lines::<String>(INPUT)?;
        let ones = count_ones(&lines);
        assert_eq!(ones, &[7, 5, 8, 7, 5]);
        assert_eq!(rates(&ones, lines.len()), (22, 9));
        assert_eq!(part1(INPUT)?, 198);
        Ok(())
    }
}

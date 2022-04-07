//! Solution for [Day 1](https://adventofcode.com/2021/day/1) puzzle.

use anyhow::{Result};
use itertools::izip;
use crate::util::{parse_lines, ToResult};

fn count_increase(data: impl Iterator<Item = usize>) -> usize {
    data.fold((usize::MAX, 0usize), |(last, count), value|
        (value, if value > last { count + 1 } else { count })
    )
    .1
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(count_increase(parse_lines::<usize>(input)?.into_iter()))
}

pub fn part2(input: &str) -> Result<usize> {
    let data = parse_lines::<usize>(input)?;
    count_increase(
        izip!(&data[0..data.len()-2], &data[1..data.len()-1], &data[2..data.len()])
        .map(|(a, b, c)| a + b + c)
    ).to_result()
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    static INPUT: &str = indoc!(
        "199
        200
        208
        210
        200
        207
        240
        269
        260
        263"
     );

    #[test]
    fn count() -> Result<()> {
        assert_eq!(part1(INPUT)?, 7);
        assert_eq!(part2(INPUT)?, 5);
        Ok(())
    }
}

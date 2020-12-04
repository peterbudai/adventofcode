//! Solution for [Day 1](https://adventofcode.com/2020/day/1) puzzle.

use anyhow::{anyhow, Result};
use itertools::Itertools;
use crate::util::parse_lines;

/// Finds the first number combination in the line separated input that adds up to 2020.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
/// * `len`: Cardinality of combinations that we are looking for.
///          Specify 2 for pairs, 3 for triples, etc.
///
fn find_combination(input: &str, len: usize) -> Result<Vec<usize>> {
    parse_lines::<usize>(input)?
    .into_iter()
    .combinations(len)
    .find(|combination| combination.iter().sum::<usize>() == 2020)
    .ok_or(anyhow!("No result"))
}

/// Finds the product of the first number combination in the line separated input
/// that adds up to 2020.
/// 
/// Multiplies the outputs of [`find_combination`](self::find_combination) together to provide the result.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
/// * `len`: Cardinality of combinations that we are looking for.
///          Specify 2 for pairs, 3 for triples, etc.
/// 
fn find_product(input: &str, len: usize) -> Result<usize> {
    find_combination(input, len)
    .map(|combination| combination.into_iter().product())
}

/// Solution for part 1: product of two entries.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
///
pub fn part1(input: &str) -> Result<usize> {
    find_product(input, 2)
}

/// Solution for part 2: product of three entries.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string.
///
pub fn part2(input: &str) -> Result<usize> {
    find_product(input, 3)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    static INPUT: &str = indoc!(
        "1721
         979
         366
         299
         675
         1456"
     );

    #[test]
    fn combination() -> Result<()> {
        assert_eq!(find_combination(INPUT, 2)?, [1721, 299]);
        assert_eq!(find_combination(INPUT, 3)?, [979, 366, 675]);
        Ok(())
    }

    #[test]
    fn product() -> Result<()> {
        assert_eq!(find_product(INPUT, 2)?, 514579);
        assert_eq!(find_product(INPUT, 3)?, 241861950);
        Ok(())
    }
}

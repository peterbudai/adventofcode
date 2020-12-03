use anyhow::{anyhow, Result};
use itertools::Itertools;
use crate::util::parse_lines;

fn find_combination(input: &str, len: usize) -> Result<Vec<usize>> {
    parse_lines::<usize>(input)?
    .into_iter()
    .combinations(len)
    .find(|combination| combination.iter().sum::<usize>() == 2020)
    .ok_or(anyhow!("No result"))
}

fn find_product(input: &str, len: usize) -> Result<usize> {
    find_combination(input, len)
    .map(|combination| combination.into_iter().product())
}

pub fn part1(input: &str) -> Result<usize> {
    find_product(input, 2)
}

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

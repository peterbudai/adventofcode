//! Utility functions and types for repeating elements of the solutions.

use std::str::FromStr;
use anyhow::Result;

/// Parse line-separated input into a vector. Can convert into anything that
/// implements the [`FromStr`](std::str::FromStr) trait.
/// 
/// # Arguments
/// 
/// * `input`: Puzzle input string
/// 
/// # Errors
/// 
/// String conversion errors from `T`.
/// 
/// # Examples
/// 
/// ```
/// # use aoc::util::parse_lines;
/// let input = parse_lines::<u32>("1\n2\n3")?;
/// assert_eq!(input, [1, 2, 3]);
/// # anyhow::Result::<()>::Ok(())
/// ```
pub fn parse_lines<'a, T>(input: &'a str) -> Result<Vec<T>, <T as FromStr>::Err> 
    where T: FromStr
{
    input.lines().map(|line| T::from_str(line)).collect()
}

#[cfg(test)]
mod test {
    use anyhow::Error;
    use super::*;

    #[test]
    fn parse_lines_primitive() -> Result<()> {
        assert_eq!(parse_lines::<char>("a\nb\nc")?, ['a','b','c']);
        assert_eq!(parse_lines::<u32>("1\n2\n")?, [1,2]);
        assert_eq!(parse_lines::<isize>("-5")?, [-5]);
        assert_eq!(parse_lines::<u8>("")?, []);
        Ok(())
    }

    #[test]
    fn parse_lines_custom() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct Pair(isize, String);

        impl FromStr for Pair {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                let parts = s.split_whitespace().collect::<Vec<_>>();
                Ok(Pair(isize::from_str(parts[0])?, parts[1].to_string()))
            }
        }

        assert_eq!(parse_lines::<Pair>("1 A\n2 B")?, [Pair(1, "A".to_string()), Pair(2, "B".to_string())]);
        Ok(())
    }
}
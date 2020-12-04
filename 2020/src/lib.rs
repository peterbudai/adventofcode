//! Solutions for [Advent of Code 2020](https://adventofcode.com/2020/) puzzles.
//! 
//! To use this library, call one of the wrapper functions to run the solutions:
//! 
//! ```
//! fn main() {
//!     aoc::day1().unwrap();
//!     // or
//!     aoc::MAIN().unwrap();
//! }
//! ```

pub mod util;

/// Generates wrapper function for reading input and printing output for each day's solution.
/// 
/// Each solution must reside in its on module and provide two functions `part1` and `part2`,
/// both with the same signature:
/// 
/// ```
/// fn part<T: Debug>(input: &str) -> anyhow::Result<T> { /* ... */ }
/// ```
/// 
/// # Examples
/// 
/// Declare 3 solutions, of which the second one will run:
/// 
/// ```
/// solution! {
///     day1,
///     day2 => main,
///     day3
/// }
/// ```
macro_rules! solution {
    ($day:ident => main) => {
        solution!($day);

        /// Default solution to run.
        /// 
        /// This is a convenience function to be called from a driver binary.
        /// 
        /// ```
        /// fn main() {
        ///     aoc::MAIN().unwrap()
        /// }
        /// ```
        pub const MAIN: fn() -> anyhow::Result<()> = $day;
    };

    ($day:ident) => {
        solution![$day, 
            concat!("Wrapper function for module [", stringify!($day), "](mod@self::", stringify!($day),").\n",
                "\n",
                "Reads puzzle input from `data/", stringify!($day), ".txt` and prints solutions to standard output.\n"
            )
        ];
    };

    [$day:ident, $doc:expr] => {
        pub mod $day;

        #[doc = $doc]
        #[allow(dead_code)]
        pub fn $day() -> anyhow::Result<()> {
            let input = std::fs::read_to_string(format!("data/{}.txt", stringify!($day)))?;
            let part1 = $day::part1(&input)?;
            let part2 = $day::part2(&input)?;
            println!("{}: {:?} {:?}", stringify!($day), part1, part2);
            Ok(())
        }
    };

    {$($day:ident $(=> $main:ident)?),+ $(,)?} => {
        $( solution!($day $(=> $main)?); )+
    };
}

solution! { 
    day1 => main,
}
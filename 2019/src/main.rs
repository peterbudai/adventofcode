mod util;
mod intcode;

macro_rules! solution {
    ($day:ident => main) => {
        mod $day;

        fn main() -> anyhow::Result<()> {
            let (a, b) = $day::solution(&std::fs::read_to_string(format!("data/{}.txt", stringify!($day)))?)?;
            println!("{}: {:} {:}", stringify!($day), a, b);
            Ok(())
        }
    };

    ($day:ident) => {
        mod $day;

        #[allow(dead_code)]
        fn $day() -> anyhow::Result<()> {
            let (a, b) = $day::solution(&std::fs::read_to_string(format!("data/{}.txt", stringify!($day)))?)?;
            println!("{}: {:} {:}", stringify!($day), a, b);
            Ok(())
        }
    };

    {$($day:ident $(=> $main:ident)?),+ $(,)?} => {
        $( solution!($day $(=> $main)?); )+
    };
}

solution! { 
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
    day7,
    day8,
    day9,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17 => main,
}
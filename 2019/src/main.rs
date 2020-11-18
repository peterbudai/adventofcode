mod util;
mod intcode;

macro_rules! solution {
    {$($day: ident),*} => {
        $(
        mod $day;

        #[allow(dead_code)]
        fn $day() -> anyhow::Result<()> {
            let (a, b) = $day::solution(&std::fs::read_to_string(format!("data/{}.txt", stringify!($day)))?)?;
            println!("{}: {:} {:}", stringify!($day), a, b);
            Ok(())
        }
        )*
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
    day13
}

fn main() -> anyhow::Result<()> {
    day13()
}

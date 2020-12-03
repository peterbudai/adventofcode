pub mod util;

macro_rules! solution {
    ($day:ident => main) => {
        solution!($day);

        pub const MAIN: fn() -> anyhow::Result<()> = $day;
    };

    ($day:ident) => {
        pub mod $day;

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
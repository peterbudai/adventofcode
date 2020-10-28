mod intcode;

macro_rules! solution {
    {$($day: ident),*} => {
        $(
        mod $day;

        #[allow(dead_code)]
        fn $day() -> anyhow::Result<()> {
            let (a, b) = $day::solution(&std::fs::read_to_string(format!("data/{}.txt", stringify!($day)))?)?;
            println!("{}: {:?} {:?}", stringify!($day), a, b);
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
    day6
}

fn main() -> anyhow::Result<()> {
    day5()
}

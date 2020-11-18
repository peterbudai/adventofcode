use anyhow::{anyhow, Result};
use std::collections::HashMap;
use crate::intcode::Computer;
use crate::util::Coord;

fn run(computer: &mut Computer, screen: &mut HashMap<Coord, isize>) -> Result<()> {
    loop {
        if !computer.run_until_output()? {
            return Ok(())
        }
        let x = computer.pop_output()?;
        if !computer.run_until_output()? {
            return Err(anyhow!("Missing output"));
        }
        let y = computer.pop_output()?;
        if !computer.run_until_output()? {
            return Err(anyhow!("Missing output"));
        }
        let tile = computer.pop_output()?;

        screen.insert((x, y), tile);
    }
}

fn draw(screen: &HashMap<Coord, isize>) -> String {
    let minx = *screen.keys().map(|(x, _)| x).min().unwrap();
    let maxx = *screen.keys().map(|(x, _)| x).max().unwrap();
    let miny = *screen.keys().map(|(_, y)| y).min().unwrap();
    let maxy = *screen.keys().map(|(_, y)| y).max().unwrap();
    let width = maxx - minx + 1;
    let height = maxy - miny + 1;

    (0..height).map(|y| 
        (0..width).map(|x|
            match *screen.get(&(minx+x, maxy-y)).unwrap_or(&0) { 
                1 => "##",
                2 => "[]",
                3 => "--",
                4 => "()",
                _ => "  ",
            }
        ).fold(String::new(), |s, p| s + p)
    ).fold(String::new(), |s, l| s + "\n" + &l)
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let mut computer = Computer::load(&data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?);
    let mut screen = HashMap::<Coord, isize>::new();
    run(&mut computer, &mut screen)?;
    println!("{}", draw(&screen));
    Ok((screen.values().filter(|t| *t == &2).count(), 0))
}

use anyhow::{anyhow, Result};
use crate::intcode::Computer;
use std::collections::HashMap;

type Coord = (isize, isize);

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Robot {
    computer: Computer,
    pos: Coord,
    dir: Direction,
}

impl Robot {
    fn new(code: &[isize]) -> Robot {
        Robot {
            computer: Computer::load(code),
            pos: (0, 0),
            dir: Direction::Up,
        }
    }

    fn run(&mut self, hull: &mut HashMap<Coord, bool>) -> Result<()> {
        loop {
            let color = *hull.get(&self.pos).unwrap_or(&false);
            self.computer.push_input(if color { 1 } else { 0 });

            if !self.computer.run_until_output()? {
                return Ok(())
            }
            let ncolor = match self.computer.pop_output()? {
                0 => false,
                1 => true,
                _ => return Err(anyhow!("Invalid color")),
            };

            if !self.computer.run_until_output()? {
                return Err(anyhow!("Missing output"));
            }
            let turn = self.computer.pop_output()?;
            let (x, y) = self.pos;
            let (ndir, nx, ny) = match self.dir {
                Direction::Up => match turn {
                    0 => (Direction::Left, x-1, y),
                    1 => (Direction::Right, x+1, y),
                    _ => return Err(anyhow!("Invalid direction")),
                },
                Direction::Left => match turn {
                    0 => (Direction::Down, x, y-1),
                    1 => (Direction::Up, x, y+1),
                    _ => return Err(anyhow!("Invalid direction")),
                },
                Direction::Down => match turn {
                    0 => (Direction::Right, x+1, y),
                    1 => (Direction::Left, x-1, y),
                    _ => return Err(anyhow!("Invalid direction")),
                },
                Direction::Right => match turn {
                    0 => (Direction::Up, x, y+1),
                    1 => (Direction::Down, x, y-1),
                    _ => return Err(anyhow!("Invalid direction")),
                },
            };

            hull.insert(self.pos, ncolor);
            self.pos = (nx, ny);
            self.dir = ndir;
        }
    }
}

fn draw(hull: &HashMap<Coord, bool>) -> String {
    let minx = *hull.keys().map(|(x, _)| x).min().unwrap();
    let maxx = *hull.keys().map(|(x, _)| x).max().unwrap();
    let miny = *hull.keys().map(|(_, y)| y).min().unwrap();
    let maxy = *hull.keys().map(|(_, y)| y).max().unwrap();
    let width = maxx - minx + 1;
    let height = maxy - miny + 1;

    (0..height).map(|y| 
        (0..width).map(|x|
            if *hull.get(&(minx+x, maxy-y)).unwrap_or(&false) { "##" } else { "  " }
        ).fold(String::new(), |s, p| s + p)
    ).fold(String::new(), |s, l| s + "\n" + &l)
}

pub fn solution(data: &str) -> Result<(usize, String)> {
    let code = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    let mut robot1 = Robot::new(&code);
    let mut hull1 = HashMap::<Coord, bool>::new();
    robot1.run(&mut hull1)?;

    let mut robot2 = Robot::new(&code);
    let mut hull2 = HashMap::<Coord, bool>::new();
    hull2.insert((0, 0), true);
    robot2.run(&mut hull2)?;

    Ok((hull1.len(), draw(&hull2)))
}

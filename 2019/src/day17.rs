use anyhow::{ensure, Result};
use crate::intcode::Computer;
use crate::util::{Coord, Dir};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    scaffold: HashSet<Coord>,
    pos: Coord,
    dir: Dir,
}

impl Map {
    fn load(data: &str) -> Result<Self> {
        let mut scaffold = HashSet::<Coord>::new();
        let mut x = 0;
        let mut y = 0;
        let mut dir = None;
        let mut pos = None;

        let mut computer = Computer::load(&data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?);
        computer.run()?;
        let mut map = Vec::<char>::new();
        while let Ok(p) = computer.pop_output() {
            map.push(p as u8 as char);
        }
        for line in map.into_iter().collect::<String>().trim().lines().rev() {
            x = 0;
            for chr in line.trim().chars().rev() {
                if chr != '.' {
                    scaffold.insert((x, y));
                    if chr != '#' {
                        pos = Some((x,y));
                        dir = match chr {
                            '^' => Some(Dir::Up),
                            'v' => Some(Dir::Down),
                            '<' => Some(Dir::Left),
                            '>' => Some(Dir::Right),
                            _ => None,
                        }
                    }
                }
                x += 1;
            }
            y += 1;
        }

        Ok(Self {
            width: x,
            height: y,
            scaffold,
            pos: pos.unwrap(),
            dir: dir.unwrap(),
        })
    }

    fn draw(&self) {
        println!();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.scaffold.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn intersections(&self) -> Vec<Coord> {
        let mut v = Vec::<Coord>::new();
        for y in 1isize..self.height-1 {
            for x in 1isize..self.width-1 {
                if self.scaffold.contains(&(x, y)) &&
                    self.scaffold.contains(&(x-1,y)) && self.scaffold.contains(&(x+1,y)) &&
                    self.scaffold.contains(&(x,y-1)) && self.scaffold.contains(&(x,y+1)) {
                    v.push((x, y));
                }
            }
        }
        v
    }
}


pub fn solution(data: &str) -> Result<(isize, usize)> {
    let map = Map::load(data)?;
    map.draw();
    Ok((map.intersections().iter().map(|(x, y)| *x * *y).sum(), 0))
}
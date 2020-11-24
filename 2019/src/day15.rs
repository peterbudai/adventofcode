use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use crate::intcode::Computer;
use crate::util::{Coord, Dir};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall = 0,
    Room = 1,
    Oxygen = 2,
}

impl TryFrom<isize> for Tile {
    type Error = Error;

    fn try_from(data: isize) -> Result<Self> {
        match data {
            0 => Ok(Tile::Wall),
            1 => Ok(Tile::Room),
            2 => Ok(Tile::Oxygen),
            _ => Err(anyhow!("Invalid tile")),
        }
    }
}

struct Droid {
    computer: Computer,
    map: HashMap<Coord, (Tile, usize)>,
    pos: Coord,
    path: Vec<Dir>,
}

impl Droid {
    fn new(code: &[isize]) -> Self {
        let pos = (0, 0);
        let mut map = HashMap::<Coord, (Tile, usize)>::new();
        map.insert(pos, (Tile::Room, 0));

        Self {
            computer: Computer::load(code),
            map,
            pos,
            path: Vec::<Dir>::new(),
        }
    }

    fn step(&mut self, dir: Dir) -> Result<Tile> {
        self.computer.push_input(dir as isize);
        if !self.computer.run_until_output()? {
            return Err(anyhow!("No output"));
        }
        let status = self.computer.pop_output()?;
        Tile::try_from(status)
    }

    fn traverse(&mut self) -> Result<()> {
        let mut state = Vec::<Vec<Dir>>::new();
        state.push(Dir::iter().collect());

        while !state.is_empty() {
            if let Some(next_dir) = state.last_mut().unwrap().pop() {
                match self.step(next_dir)? {
                    t @ Tile::Wall => {
                        self.map.insert(next_dir.apply(&self.pos), (t, self.path.len()));
                    },
                    t @ Tile::Room | t @ Tile::Oxygen => {
                        next_dir.apply_mut(&mut self.pos);
                        self.path.push(next_dir);
                        if let Some((_, p)) = self.map.get_mut(&self.pos) {
                            if *p > self.path.len() {
                                *p = self.path.len();
                            }
                        } else {
                            self.map.insert(self.pos, (t, self.path.len()));
                        }
                        state.push(Dir::iter().filter(|d| d != &next_dir.opposite()).collect());
                    },
                }
            } else {
                state.pop();
                if let Some(back_dir) = self.path.pop() {
                    self.step(back_dir.opposite())?;
                    back_dir.opposite().apply_mut(&mut self.pos);
                }
            }
        }
        Ok(())
    }

    fn draw(&self) {
        let minx = *self.map.keys().map(|(x, _)| x).min().unwrap();
        let maxx = *self.map.keys().map(|(x, _)| x).max().unwrap();
        let miny = *self.map.keys().map(|(_, y)| y).min().unwrap();
        let maxy = *self.map.keys().map(|(_, y)| y).max().unwrap();
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
    
        let image = (0..height).map(|y| 
            (0..width).map(|x|
                match self.map.get(&(minx+x, maxy-y)) { 
                    Some((Tile::Wall, _)) => "##",
                    Some((Tile::Room, _)) if self.pos == (minx+x, maxy-y) => "<>",
                    Some((Tile::Room, _)) => "..",
                    Some((Tile::Oxygen, _)) => "()",
                    None => "  ",
                }
            ).fold(String::new(), |s, p| s + p)
        ).fold(String::new(), |s, l| s + "\n" + &l);

        println!("{}", image);
    }

    fn oxygen_distance(&self) -> usize {
        self.map.values().find_map(|(t, p)| if t == &Tile::Oxygen { Some(*p) } else { None }).unwrap()
    }

    fn oxygen_fill_time(&mut self) -> usize {
        let mut max_round = 0;
        let mut coords = self.map.iter_mut().filter_map(|(c, (t, _))| if t == &Tile::Oxygen { Some ((*c, 0)) } else { None }).collect::<Vec<_>>();
        while !coords.is_empty() {
            let (coord, round) = coords.remove(0);
            for dir in Dir::iter() {
                let new_coord = dir.apply(&coord);
                if let Some((tile @ Tile::Room, _)) = self.map.get_mut(&new_coord) {
                    *tile = Tile::Oxygen;
                    coords.push((new_coord, round+1));
                }
            }
            if round > max_round {
                max_round = round;
            }
        }
        max_round
    }
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let code = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;
    let mut droid = Droid::new(&code);
    droid.traverse()?;
    droid.draw();
    Ok((droid.oxygen_distance(), droid.oxygen_fill_time()))
}

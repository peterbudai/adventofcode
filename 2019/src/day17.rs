use anyhow::Result;
use crate::intcode::Computer;
use crate::util::{Coord, Dir};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    scaffold: HashMap<Coord, bool>,
    pos: Coord,
    dir: Dir,
}

impl Map {
    fn load(mut computer: Computer) -> Result<Self> {
        let mut scaffold = HashMap::<Coord, bool>::new();
        let mut x = 0;
        let mut y = 0;
        let mut dir = None;
        let mut pos = None;

        computer.run()?;
        let mut map = Vec::<char>::new();
        while let Ok(p) = computer.pop_output() {
            map.push(p as u8 as char);
        }
        for line in map.into_iter().collect::<String>().trim().lines().rev() {
            x = 0;
            for chr in line.trim().chars().rev() {
                if chr != '.' {
                    scaffold.insert((x, y), chr != '#');
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

    #[cfg(debug)]
    fn draw(&self) {
        println!();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.scaffold.contains_key(&(x, y)) {
                    if self.pos == (x, y) {
                        print!("{}", match self.dir {
                            Dir::Up => '^',
                            Dir::Down => 'v',
                            Dir::Left => '<',
                            Dir::Right => '>',
                        });
                    } else {
                        print!("{}", if let Some(true) = self.scaffold.get(&(x, y)) { '@' } else { '#' });
                    }
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
                if self.scaffold.contains_key(&(x, y)) &&
                    self.scaffold.contains_key(&(x-1,y)) && self.scaffold.contains_key(&(x+1,y)) &&
                    self.scaffold.contains_key(&(x,y-1)) && self.scaffold.contains_key(&(x,y+1)) {
                    v.push((x, y));
                }
            }
        }
        v
    }

    fn next_pos(&self) -> Option<(Option<bool>, Dir, Coord)> {
        let fw_pos = self.dir.apply(&self.pos);
        if self.scaffold.contains_key(&fw_pos) {
            return Some((None, self.dir, fw_pos));
        }

        for turn in [true, false].iter() {
            let t_dir = self.dir.turn(*turn);
            let t_pos = t_dir.apply(&self.pos);
            if self.scaffold.contains_key(&t_pos) {
                return Some((Some(*turn), t_dir, t_pos));
            }
        }

        let o_dir = self.dir.opposite();
        let o_pos = o_dir.apply(&self.pos);
        if let Some(visited) = self.scaffold.get(&o_pos) {
            if !visited {
                return Some((Some(true), self.dir.turn(true), self.pos));
            }
        }

        None
    }

    fn path(&mut self) -> Vec<(char, usize)> {
        let mut path = Vec::<(char, usize)>::new();
        let mut forward = 0;
        let mut last_turn = None;
        while let Some((turn, ndir, npos)) = self.next_pos() {
            if let Some(cw) = turn {
                if forward > 0 {
                    path.push((last_turn.unwrap(), forward));
                    forward = 0;
                }
                last_turn = Some(if cw {'R'} else {'L'});
            }
            self.dir = ndir;
            if npos != self.pos {
                forward += 1;
                *self.scaffold.get_mut(&npos).unwrap() = true;
                self.pos = npos;
            }
        }
        if forward > 0 {
            path.push((last_turn.unwrap(), forward));
        }
        path
    }
}

fn find_match(source: &[(char, usize)], bitmap: &[bool], from: usize, start: usize, len: usize) -> bool {
    for i in 0..len {
        if !bitmap[from + i] || !bitmap[start + i] || source[from + i] != source[start + i] {
            return false;
        }
    }
    true
}

fn find_repeats_from(source: &[(char, usize)], bitmap: &[bool], from: usize, len: usize) -> Vec<usize> {
    let mut start = from;
    let mut repeats = Vec::<usize>::new();
    while start + len <= source.len() {
        if find_match(source, bitmap, from, start, len) {
            repeats.push(start);
            start += len;
        } else {
            start += 1;
        }
    }
    repeats
}

fn longest_repeat_from(source: &[(char, usize)], bitmap: &[bool], from: usize) -> (usize, Vec<usize>) {
    let mut len = source.len() - from;
    while len > 1 {
        let repeats = find_repeats_from(source, bitmap, from, len);
        if repeats.len() > 1 {
            return (len, repeats);
        }
        len -= 1;
    }
    (0, Vec::<usize>::new())
}

fn find_repeats(source: &[(char, usize)]) -> Vec<(usize, Vec<usize>)> {
    let mut bitmap = vec![true; source.len()];
    let mut repeats = Vec::<(usize, Vec<usize>)>::new();
    while let Some((from, _)) = bitmap.iter().enumerate().find(|(_, b)| **b) {
        let (len, starts) = longest_repeat_from(source, &bitmap, from);
        for s in &starts {
            for i in 0..len {
                *bitmap.get_mut(s+i).unwrap() = false;
            }
        }
        repeats.push((len, starts));
    }
    repeats
}

fn create_programs(source: &[(char, usize)], repeats: &[(usize, Vec<usize>)]) -> Vec<String> {
    let mut programs = Vec::<String>::new();

    programs.push(
        repeats.iter()
        .enumerate()
        .map(|(sub,(_, poslist))| 
            poslist.iter().map(|pos| (pos, sub)).collect_vec()
        )
        .flatten()
        .sorted_by_key(|(pos, _)| *pos)
        .map(|(_, sub)| ('A' as u8 + sub as u8) as char)
        .join(",")
    );
    programs.append(&mut
        repeats.iter()
        .map(|(len, poslist)| &source[poslist[0]..poslist[0]+len])
        .map(|steplist| steplist.iter().flat_map(|(turn, forward)| vec![turn.to_string(), forward.to_string()]).join(","))
        .collect_vec()
    );
    programs
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let mut computer = Computer::load(&data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?);

    let mut map = Map::load(computer.clone())?;
    let path = map.path();
    let repeats = find_repeats(&path);
    let lines = create_programs(&path, &repeats);

    computer.set_control_word(2);
    for c in (lines.join("\n") + "\nn\n").chars() {
        computer.push_input(c as isize);
    }
    computer.run()?;

    Ok((map.intersections().iter().map(|(x, y)| *x * *y).sum(), computer.pop_output()?))
}
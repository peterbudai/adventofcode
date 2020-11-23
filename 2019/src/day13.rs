use anyhow::{anyhow, Result};
use std::collections::HashMap;
use crate::intcode::Computer;
use crate::util::Coord;

struct Game {
    computer: Computer,
    screen: HashMap<Coord, isize>,
    blocks: usize,
    score: usize,
    paddle: Option<isize>,
    ball: Option<(isize, isize)>,
}

impl Game {
    fn without_coin(code: &[isize]) -> Self {
        Self {
            computer: Computer::load(code),
            screen: HashMap::<Coord, isize>::new(),
            blocks: 0,
            score: 0,
            paddle: None,
            ball: None,
        }
    }

    fn with_coin(code: &[isize]) -> Self {
        let mut hacked = code.to_owned();
        hacked[0] = 2;
        Self::without_coin(&hacked)
    }

    fn draw(&self) {
        let minx = *self.screen.keys().map(|(x, _)| x).min().unwrap();
        let maxx = *self.screen.keys().map(|(x, _)| x).max().unwrap();
        let miny = *self.screen.keys().map(|(_, y)| y).min().unwrap();
        let maxy = *self.screen.keys().map(|(_, y)| y).max().unwrap();
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
    
        let image = (0..height).map(|y| 
            (0..width).map(|x|
                match *self.screen.get(&(minx+x, maxy-y)).unwrap_or(&0) { 
                    1 => "##",
                    2 => "[]",
                    3 => "--",
                    4 => "()",
                    _ => "  ",
                }
            ).fold(String::new(), |s, p| s + p)
        ).fold(String::new(), |s, l| s + "\n" + &l);

        println!("\n|{}|{}", self.score, image);
    }

    fn run_step(&mut self) -> Result<bool> {
        if !self.computer.run_until_output()? {
            return Ok(false);
        }
        let x = self.computer.pop_output()?;
        if !self.computer.run_until_output()? {
            return Err(anyhow!("Missing output"));
        }
        let y = self.computer.pop_output()?;
        if !self.computer.run_until_output()? {
            return Err(anyhow!("Missing output"));
        }
        let tile = self.computer.pop_output()?;

        if (x, y) == (-1, 0) {
            self.score = tile as usize;
            return Ok(true);
        }
        match tile {
            2 => { self.blocks += 1; },
            3 => { self.paddle = Some(x); },
            4 => { self.ball = Some((x, y)); },
            _ => {},
        }
        if let Some(old_tile) = self.screen.insert((x, y), tile) {
            if old_tile == 2 && tile == 0 {
                self.blocks -= 1;
            }
        }
        Ok(true)
    }
    
    fn run(&mut self) -> Result<()> {
        while self.run_step()? {}
        self.draw();
        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        loop {
            if !self.run_step()? {
                self.draw();
                return Ok(());
            }

            if let Some(paddle_x) = self.paddle {
                if let Some((ball_x, _)) = self.ball {
                    let movement = (ball_x - paddle_x).signum();
                    self.computer.push_input(movement);
                    if movement != 0 {
                        self.paddle = None;
                    }
                    self.ball = None;
                }
            }
        }
    }
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let code = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    let mut game1 = Game::without_coin(&code);
    game1.run()?;

    let mut game2 = Game::with_coin(&code);
    game2.play()?;

    Ok((game1.blocks, game2.score))
}

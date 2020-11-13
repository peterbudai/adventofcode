use anyhow::Result;
use crate::intcode::Computer;

fn run_program(computer: &Computer, program: isize) -> Result<isize> {
    let mut c = computer.clone();
    c.push_input(program);
    c.run()?;
    c.pop_output()
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let computer = Computer::load(&data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?);
    Ok((run_program(&computer, 1)?, run_program(&computer, 2)?))
}

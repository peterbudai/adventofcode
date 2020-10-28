use anyhow::Result;
use crate::intcode::Computer;

fn run_input(code: &[isize], input: isize) -> Result<isize> {
    let mut computer = Computer::load(code);
    computer.set_input(&[input]);
    computer.run()?;
    computer.get_output()
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let code  = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    Ok((run_input(&code, 1)?, run_input(&code, 5)?))
}

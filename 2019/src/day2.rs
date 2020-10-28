use anyhow::Result;
use crate::intcode::Computer;

fn run_verb_noun(code: &[isize], noun: isize, verb: isize) -> Result<isize> {
    let mut computer = Computer::load(code);
    computer.set_noun_verb(noun, verb);
    computer.run()?;
    Ok(computer.get_result())
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let code  = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;
    let output = run_verb_noun(&code, 12, 2)?;

    for noun in 0..100 {
        for verb in 0..100 {
            if run_verb_noun(&code, noun, verb)? == 19690720 {
                return Ok((output, 100 * noun + verb))
            }
        }
    }
    anyhow::bail!("No result")
}

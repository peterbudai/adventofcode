use anyhow::Result;
use itertools::Itertools;
use crate::intcode::Computer;

fn run_amplifiers(code: &[isize], phase_sequence: &[isize]) -> Result<isize> {
    let computer = Computer::load(code);
    let mut signal = 0;
    for phase in phase_sequence {
        let mut c = computer.clone();
        c.set_input(&[*phase, signal]);
        c.run()?;
        signal = c.get_output()?
    }
    Ok(signal)
}

#[cfg(test)]
#[test]
fn test_run_amplifiers() {
    assert_eq!(run_amplifiers(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &[4,3,2,1,0]).unwrap(), 43210);
    assert_eq!(run_amplifiers(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], &[0,1,2,3,4]).unwrap(), 54321);
    assert_eq!(run_amplifiers(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], &[1,0,4,3,2]).unwrap(), 65210);
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let code  = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    Ok(((0..5).into_iter().permutations(5).map(|p| run_amplifiers(&code, &p).unwrap()).max().unwrap(),0))
}

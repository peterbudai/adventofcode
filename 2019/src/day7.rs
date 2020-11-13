use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::ops::Range;
use crate::intcode::Computer;

fn run_amplifiers_oneshot(code: &[isize], phase_sequence: &[isize]) -> Result<isize> {
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

fn run_amplifiers_feedback(code: &[isize], phase_sequence: &[isize]) -> Result<isize> {
    let mut computers = vec![Computer::load(code); phase_sequence.len()];
    computers.iter_mut().zip(phase_sequence.iter()).for_each(|(c,p)| c.push_input(*p));

    let mut signal = 0;
    let mut output = true;
    while output {
        for c in &mut computers {
            if output {
                c.push_input(signal);
            }
            output = c.run_until_output()?;
            if output {
                signal = c.pop_output()?;
            }
        }
    }
    Ok(signal)
}

fn find_max(code: &[isize], phases: Range<isize>, run_amp_fn: fn(&[isize], &[isize])->Result<isize>) -> Result<isize> {
    phases.into_iter()
        .permutations(5)
        .map(|p| run_amp_fn(&code, &p))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .ok_or(anyhow!("No result"))
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let code  = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    Ok((
        find_max(&code, 0..5, run_amplifiers_oneshot)?,
        find_max(&code, 5..10, run_amplifiers_feedback)?
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn amplifiers_oneshot() {
        let c1 = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(run_amplifiers_oneshot(&c1, &[4,3,2,1,0]).unwrap(), 43210);
        assert_eq!(find_max(&c1, 0..5, run_amplifiers_oneshot).unwrap(), 43210);

        let c2 = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(run_amplifiers_oneshot(&c2, &[0,1,2,3,4]).unwrap(), 54321);
        assert_eq!(find_max(&c2, 0..5, run_amplifiers_oneshot).unwrap(), 54321);

        let c3 = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(run_amplifiers_oneshot(&c3, &[1,0,4,3,2]).unwrap(), 65210);
        assert_eq!(find_max(&c3, 0..5, run_amplifiers_oneshot).unwrap(), 65210);
    }

    #[test]
    fn amplifiers_feedback() {
        let c1 = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(run_amplifiers_feedback(&c1, &[9,8,7,6,5]).unwrap(), 139629729);
        assert_eq!(find_max(&c1, 5..10, run_amplifiers_feedback).unwrap(), 139629729);

        let c2 = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        assert_eq!(run_amplifiers_feedback(&c2, &[9,7,8,5,6]).unwrap(), 18216);
        assert_eq!(find_max(&c2, 5..10, run_amplifiers_feedback).unwrap(), 18216);
    }
}
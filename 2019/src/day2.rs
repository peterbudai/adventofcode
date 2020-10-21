use anyhow::Result;

fn run_intcode(code: &mut [usize]) {
    let mut ip = 0usize;
    loop {
        let inst = code[ip];
        match inst {
            99 => return,
            1 | 2 => { 
                let pos = code[ip+3];
                let res = if inst == 1 { 
                    code[code[ip+1]] + code[code[ip+2]]
                } else {
                    code[code[ip+1]] * code[code[ip+2]]
                };
                code[pos] = res;            
            },
            _ => unreachable!()
        }
        ip += 4;
    }
}

#[cfg(test)]
#[test]
fn test_run_intcode() {
    let mut code = vec![1,0,0,0,99];
    run_intcode(&mut code);
    assert_eq!(code, &[2,0,0,0,99]);

    code = vec![2,3,0,3,99];
    run_intcode(&mut code);
    assert_eq!(code, &[2,3,0,6,99]);

    code = vec![2,4,4,5,99,0];
    run_intcode(&mut code);
    assert_eq!(code, &[2,4,4,5,99,9801]);

    code = vec![1,1,1,4,99,5,6,0,99];
    run_intcode(&mut code);
    assert_eq!(code, &[30,1,1,4,2,5,6,0,99]);

    code = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    run_intcode(&mut code);
    assert_eq!(code, &[3500,9,10,70,2,3,11,0,99,30,40,50]);
}

fn run_verb_noun(code: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = code.to_owned();
    memory[1] = noun;
    memory[2] = verb;
    run_intcode(&mut memory);
    memory[0]
}

pub fn solution(data: &str) -> Result<(usize, usize)> {
    let code  = data.split(',').map(|s| s.parse::<usize>()).collect::<Result<Vec<_>,_>>()?;
    let output = run_verb_noun(&code, 12, 2);

    for noun in 0..100 {
        for verb in 0..100 {
            if run_verb_noun(&code, noun, verb) == 19690720 {
                return Ok((output, 100 * noun + verb))
            }

        }
    }
    anyhow::bail!("No result")
}

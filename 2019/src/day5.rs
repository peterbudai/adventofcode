use anyhow::Result;

fn immediate(op: isize, pid: u32) -> bool {
    op / 10isize.pow(pid + 1) % 10 != 0
}

#[cfg(test)]
#[test]
fn test_immediate() {
    assert!(!immediate(1002, 1));
    assert!(immediate(1002, 2));
    assert!(!immediate(1002, 3));
}

fn read<'a>(memory: &'a [isize], ip: usize, pid: u32) -> &'a isize {
    if immediate(memory[ip], pid) {
        &memory[ip + pid as usize]
    } else {
        &memory[memory[ip + pid as usize] as usize]
    }
}

fn write<'a>(memory: &'a mut [isize], ip: usize, pid: u32) -> &'a mut isize {
    if immediate(memory[ip], pid) {
        &mut memory[ip + pid as usize]
    } else {
        &mut memory[memory[ip + pid as usize] as usize]
    }
}

#[cfg(test)]
#[test]
fn test_read_write() {
    let mut memory = vec![1002,4,3,4,33];

    assert_eq!(*read(&memory, 0, 1), 33);
    assert_eq!(*read(&memory, 0, 2), 3);
    assert_eq!(*read(&memory, 0, 3), 33);

    assert_eq!(*write(&mut memory, 0, 1), 33);
    assert_eq!(*write(&mut memory, 0, 2), 3);
    assert_eq!(*write(&mut memory, 0, 3), 33);
}

fn run_intcode(memory: &mut [isize], input: &mut Vec<isize>) -> Vec<isize> {
    let mut ip = 0usize;
    let mut output = vec![0isize; 0];
    loop {
        ip += match memory[ip] % 100 {
            1 => {
                *write(memory, ip, 3) = *read(memory, ip, 1) + *read(memory, ip, 2);
                4
            },
            2 => { 
                *write(memory, ip, 3) = *read(memory, ip, 1) * *read(memory, ip, 2);
                4
            },
            3 => { 
                *write(memory, ip, 1) = input.remove(0);
                2
            },
            4 => { 
                output.push(*read(memory, ip, 1));
                2
            },
            5 => if *read(memory, ip, 1) != 0 {
                ip = *read(memory, ip, 2) as usize;
                0
            } else {
                3
            },
            6 => if *read(memory, ip, 1) == 0 {
                ip = *read(memory, ip, 2) as usize;
                0
            } else {
                3
            },
            7 => {
                *write(memory, ip, 3) = if *read(memory, ip, 1) < *read(memory, ip, 2) { 1 } else { 0 };
                4
            },
            8 => {
                *write(memory, ip, 3) = if *read(memory, ip, 1) == *read(memory, ip, 2) { 1 } else { 0 };
                4
            },
            99 => break,
            _ => unreachable!()
        }
    }
    output
}

#[cfg(test)]
#[test]
fn test_run_param_modes() {
    let mut input = vec![];
    let mut memory = vec![1002,4,3,4,33];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[1002,4,3,4,99]);
}

#[cfg(test)]
#[test]
fn test_run_arithmetic() {
    let mut input = vec![];

    let mut memory = vec![1,0,0,0,99];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[2,0,0,0,99]);

    memory = vec![2,3,0,3,99];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[2,3,0,6,99]);

    memory = vec![2,4,4,5,99,0];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[2,4,4,5,99,9801]);

    memory = vec![1,1,1,4,99,5,6,0,99];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[30,1,1,4,2,5,6,0,99]);

    memory = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[3500,9,10,70,2,3,11,0,99,30,40,50]);
}

#[cfg(test)]
#[test]
fn test_run_input_output() {
    let mut input = vec![7];
    let mut memory = vec![3,0,4,0,99];
    let output = run_intcode(&mut memory, &mut input);
    assert_eq!(memory, &[7,0,4,0,99]);
    assert_eq!(input, &[]);
    assert_eq!(output, &[7]);
}

#[cfg(test)]
#[test]
fn test_run_branching() {
    let mut input = vec![7,8];
    let mut memory = vec![3,9,8,9,10,9,4,9,99,-1,8];

    let mut output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[8]);
    assert_eq!(output, &[0]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[1]);

    input = vec![7,8,9];
    memory = vec![3,9,7,9,10,9,4,9,99,-1,8];

    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[8,9]);
    assert_eq!(output, &[1]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[9]);
    assert_eq!(output, &[0]);

    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[0]);

    input = vec![7,8];
    memory = vec![3,3,1108,-1,8,3,4,3,99];

    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[8]);
    assert_eq!(output, &[0]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[1]);

    input = vec![7,8,9];
    memory = vec![3,3,1107,-1,8,3,4,3,99];

    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[8,9]);
    assert_eq!(output, &[1]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[9]);
    assert_eq!(output, &[0]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[0]);

    input = vec![-1,0,3];

    memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[0,3]);
    assert_eq!(output, &[1]);
    memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[3]);
    assert_eq!(output, &[0]);
    memory = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[1]);

    input = vec![-1,0,3];

    memory = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[0,3]);
    assert_eq!(output, &[1]);
    memory = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[3]);
    assert_eq!(output, &[0]);
    memory = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[1]);

    input = vec![7,8,9];
    memory = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[8,9]);
    assert_eq!(output, &[999]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[9]);
    assert_eq!(output, &[1000]);
    output = run_intcode(&mut memory, &mut input);
    assert_eq!(input, &[]);
    assert_eq!(output, &[1001]);
}

fn run_input(code: &[isize], input: isize) -> isize {
    let mut memory = code.to_owned();
    let mut input = vec![input];
    *run_intcode(&mut memory, &mut input).last().unwrap()
}

pub fn solution(data: &str) -> Result<(isize, isize)> {
    let code  = data.split(',').map(|s| s.parse::<isize>()).collect::<Result<Vec<_>,_>>()?;

    Ok((run_input(&code, 1), run_input(&code, 5)))
}

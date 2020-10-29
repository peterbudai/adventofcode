use anyhow::{anyhow, ensure, Result};

#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<isize>,
    ip: usize,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Computer {
    pub fn load(code: &[isize]) -> Self {
        Computer { 
            memory: code.to_owned(),
            ip: 0,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn push_input(&mut self, data: isize) {
        self.input.push(data);
    }

    pub fn set_input(&mut self, data: &[isize]) {
        self.input = data.to_owned();
    }

    pub fn set_noun_verb(&mut self, noun: isize, verb: isize) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }
    
    pub fn pop_output(&mut self) -> Result<isize> {
        self.output.pop().ok_or(anyhow!("Empty output"))
    }

    pub fn get_output(&self) -> Result<isize> {
        self.output.last().copied().ok_or(anyhow!("Empty output"))
    }

    pub fn get_result(&self) -> isize {
        self.memory[0]
    }
    
    fn current_opcode(&self) -> Result<isize> {
        ensure!(self.ip < self.memory.len(), "Program overrun");
        Ok(self.memory[self.ip] % 100)
    }
    
    fn is_param_immediate(&self, param_idx: u32) -> bool {
        self.memory[self.ip] / 10isize.pow(param_idx + 1) % 10 != 0
    }

    fn param(&self, idx: u32) -> Result<&isize> {
        ensure!(self.ip + (idx as usize) < self.memory.len(), "Out of bounds");

        if self.is_param_immediate(idx) {
            Ok(&self.memory[self.ip + idx as usize])
        } else {
            let p = self.memory[self.ip + idx as usize];
            Ok(&self.memory[p as usize])
        }
    }
    
    fn param_mut(&mut self, idx: u32) -> Result<&mut isize> {
        ensure!(self.ip + (idx as usize) < self.memory.len(), "Out of bounds");

        if self.is_param_immediate(idx) {
            Err(anyhow!("Invalid parameter mode"))
        } else {
            let p = self.memory[self.ip + idx as usize];
            Ok(&mut self.memory[p as usize])
        }
    }

    pub fn run_single_step(&mut self) -> Result<bool> {
        self.ip += match self.current_opcode()? {
            1 => {
                *self.param_mut(3)? = *self.param(1)? + *self.param(2)?;
                4
            },
            2 => { 
                *self.param_mut(3)? = *self.param(1)? * *self.param(2)?;
                4
            },
            3 => { 
                *self.param_mut(1)? = *self.input.first().ok_or(anyhow!("Empty input"))?;
                self.input.remove(0);
                2
            },
            4 => { 
                self.output.push(*self.param(1)?);
                2
            },
            5 => if *self.param(1)? != 0 {
                self.ip = *self.param(2)? as usize;
                0
            } else {
                3
            },
            6 => if *self.param(1)? == 0 {
                self.ip = *self.param(2)? as usize;
                0
            } else {
                3
            },
            7 => {
                *self.param_mut(3)? = if *self.param(1)? < *self.param(2)? { 1 } else { 0 };
                4
            },
            8 => {
                *self.param_mut(3)? = if *self.param(1)? == *self.param(2)? { 1 } else { 0 };
                4
            },
            99 => return Ok(false),
            _ => return Err(anyhow!("Invalid opcode"))
        };
        Ok(true)
    }

    pub fn run_until_output(&mut self) -> Result<bool> {
        let output_len = self.output.len();
        while self.run_single_step()? {
            if self.output.len() > output_len {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn run(&mut self) -> Result<()> {
        while self.run_single_step()? {
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Computer;
    
    #[test]
    fn immediate() {
        let c = Computer::load(&[1002]);
        assert!(!c.is_param_immediate(1));
        assert!(c.is_param_immediate(2));
        assert!(!c.is_param_immediate(3));
    }
    
    #[test]
    fn param_modes() {
        let mut c = Computer::load(&[1002,4,3,4,33]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[1002,4,3,4,99]);
    }

    #[test]
    fn memory_arithmetic() {
        let mut c = Computer::load(&[1,0,0,0,99]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[2,0,0,0,99]);

        c = Computer::load(&[2,3,0,3,99]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[2,3,0,6,99]);

        c = Computer::load(&[2,4,4,5,99,0]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[2,4,4,5,99,9801]);

        c = Computer::load(&[1,1,1,4,99,5,6,0,99]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[30,1,1,4,2,5,6,0,99]);

        c = Computer::load(&[1,9,10,3,2,3,11,0,99,30,40,50]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[3500,9,10,70,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn memory_read_write() {
        let mut c = Computer::load(&[1002,4,3,4,33]);
    
        assert_eq!(*c.param(1).unwrap(), 33);
        assert_eq!(*c.param(2).unwrap(), 3);
        assert_eq!(*c.param(3).unwrap(), 33);
    
        assert_eq!(*c.param_mut(1).unwrap(), 33);
        assert!(c.param_mut(2).is_err());
        assert_eq!(*c.param_mut(3).unwrap(), 33);

        *c.param_mut(1).unwrap() = 34;
        *c.param_mut(3).unwrap() = 35;

        assert_eq!(*c.param(1).unwrap(), 35);
        assert_eq!(*c.param(2).unwrap(), 3);
        assert_eq!(*c.param(3).unwrap(), 35);
    }

    #[test]
    fn input_output() {
        let mut c = Computer::load(&[3,0,4,0,99]);
        c.set_input(&[7]);
        assert!(c.run().is_ok());
        assert_eq!(c.memory, &[7,0,4,0,99]);
        assert_eq!(c.input, &[]);
        assert_eq!(c.output, &[7]);
        assert_eq!(c.get_output().unwrap(), 7);
    }

    #[test]
    fn test_run_branching() {
        let mut b = Computer::load(&[3,9,8,9,10,9,4,9,99,-1,8]);

        let mut c = b.clone();
        c.set_input(&[7]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[8]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 1);
    
        b = Computer::load(&[3,9,7,9,10,9,4,9,99,-1,8]);
    
        let mut c = b.clone();
        c.set_input(&[7]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 1);
        let mut c = b.clone();
        c.set_input(&[8]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[9]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
    
        b = Computer::load(&[3,3,1108,-1,8,3,4,3,99]);
    
        let mut c = b.clone();
        c.set_input(&[7]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[8]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 1);
    
        b = Computer::load(&[3,3,1107,-1,8,3,4,3,99]);
    
        let mut c = b.clone();
        c.set_input(&[7]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 1);
        let mut c = b.clone();
        c.set_input(&[8]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[9]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.get_output().unwrap(), 0);
    
        b = Computer::load(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);

        let mut c = b.clone();
        c.set_input(&[-1]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 1);
        let mut c = b.clone();
        c.set_input(&[-0]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[3]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 1);
    
        b = Computer::load(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);

        let mut c = b.clone();
        c.set_input(&[-1]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 1);
        let mut c = b.clone();
        c.set_input(&[0]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 0);
        let mut c = b.clone();
        c.set_input(&[3]);
        assert!(c.run().is_ok());
        assert_eq!(c.get_output().unwrap(), 1);
    
        b = Computer::load(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);

        let mut c = b.clone();
        c.set_input(&[7]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.output, &[999]);
        let mut c = b.clone();
        c.set_input(&[8]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.output, &[1000]);
        let mut c = b.clone();
        c.set_input(&[9]);
        assert!(c.run_until_output().unwrap());
        assert_eq!(c.output, &[1001]);
    }
}
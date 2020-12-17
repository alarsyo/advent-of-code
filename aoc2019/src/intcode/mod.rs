use anyhow::{anyhow, bail, Result};

mod parameter;

use parameter::Parameter;

pub fn parse_memory(s: &str) -> Result<Vec<i64>> {
    s.trim_end()
        .split(',')
        .map(|x| x.parse().map_err(anyhow::Error::new))
        .collect()
}

#[derive(Debug)]
enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustRelBase(Parameter),
    Halt,
}

#[derive(Debug)]
pub struct Intcode {
    pub memory: Vec<i64>,
    input: Vec<i64>,
    pub output: Vec<i64>,
    ip: usize,
    next_input: usize,
    wait_input: bool,
    relative_base: usize,
}

impl Intcode {
    pub fn new(input: &str) -> Result<Self> {
        let memory = parse_memory(input)?;

        Ok(Intcode::with_memory(memory))
    }

    pub fn with_memory(memory: Vec<i64>) -> Self {
        Intcode {
            memory,
            input: Vec::new(),
            output: Vec::new(),
            ip: 0,
            next_input: 0,
            wait_input: false,
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, value: i64) {
        self.input.push(value);
    }

    fn get_opcode(&self) -> Result<Opcode> {
        let instruction = self.memory[self.ip];

        let opcode = instruction % 100;
        let mode1 = instruction / 100;
        let mode2 = instruction / 1000;
        let mode3 = instruction / 10000;
        match opcode {
            1 => {
                let op1 = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let op2 = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;
                let dst = Parameter::new(mode3, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(anyhow!("add: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Add(op1, op2, dst))
                }
            }
            2 => {
                let op1 = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let op2 = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;
                let dst = Parameter::new(mode3, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(anyhow!(
                        "multiply: destination parameter can't be immediate"
                    ))
                } else {
                    Ok(Opcode::Multiply(op1, op2, dst))
                }
            }
            3 => {
                let dst = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(anyhow!("input: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Input(dst))
                }
            }
            4 => {
                let op = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;

                Ok(Opcode::Output(op))
            }
            5 => {
                let test = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let dst = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;

                Ok(Opcode::JumpTrue(test, dst))
            }
            6 => {
                let test = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let dst = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;

                Ok(Opcode::JumpFalse(test, dst))
            }
            7 => {
                let op1 = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let op2 = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;
                let dst = Parameter::new(mode3, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(anyhow!(
                        "less than: destination parameter can't be immediate"
                    ))
                } else {
                    Ok(Opcode::LessThan(op1, op2, dst))
                }
            }
            8 => {
                let op1 = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;
                let op2 = Parameter::new(mode2, self.memory.get(self.ip + 2).copied())?;
                let dst = Parameter::new(mode3, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(anyhow!("equals: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Equals(op1, op2, dst))
                }
            }
            9 => {
                let offset = Parameter::new(mode1, self.memory.get(self.ip + 1).copied())?;

                Ok(Opcode::AdjustRelBase(offset))
            }
            99 => Ok(Opcode::Halt),
            _ => Err(anyhow!("unknown opcode: {}", opcode)),
        }
    }

    fn exec(&mut self) -> Result<bool> {
        loop {
            if self.ip >= self.memory.len() {
                bail!("reached end of program without halting");
            }

            let opcode = self.get_opcode()?;

            match opcode {
                Opcode::Add(op1, op2, dst) => {
                    let val1 = op1.get(&mut self.memory, self.relative_base)?;
                    let val2 = op2.get(&mut self.memory, self.relative_base)?;

                    dst.set(val1 + val2, &mut self.memory, self.relative_base)?;

                    self.ip += 4;
                }
                Opcode::Multiply(op1, op2, dst) => {
                    let val1 = op1.get(&mut self.memory, self.relative_base)?;
                    let val2 = op2.get(&mut self.memory, self.relative_base)?;

                    dst.set(val1 * val2, &mut self.memory, self.relative_base)?;

                    self.ip += 4;
                }
                Opcode::Input(dst) => {
                    let input = if self.next_input < self.input.len() {
                        let res = self.input[self.next_input];
                        self.next_input += 1;
                        res
                    } else if self.wait_input {
                        break Ok(false);
                    } else {
                        break Err(anyhow!("tried to read input but it was empty"));
                    };
                    dst.set(input, &mut self.memory, self.relative_base)?;

                    self.ip += 2;
                }
                Opcode::Output(op) => {
                    let val = op.get(&mut self.memory, self.relative_base)?;
                    self.output.push(val);

                    self.ip += 2;
                }
                Opcode::JumpTrue(test, dst) => {
                    let val = test.get(&mut self.memory, self.relative_base)?;
                    let dst = dst.get(&mut self.memory, self.relative_base)?;
                    if dst < 0 {
                        bail!("dst must be a valid address: {}", dst);
                    }

                    if val == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = dst as usize;
                    }
                }
                Opcode::JumpFalse(test, dst) => {
                    let val = test.get(&mut self.memory, self.relative_base)?;
                    let dst = dst.get(&mut self.memory, self.relative_base)?;
                    if dst < 0 {
                        bail!("dst must be a valid address: {}", dst);
                    }

                    if val == 0 {
                        self.ip = dst as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Opcode::LessThan(op1, op2, dst) => {
                    let val1 = op1.get(&mut self.memory, self.relative_base)?;
                    let val2 = op2.get(&mut self.memory, self.relative_base)?;

                    let res = if val1 < val2 { 1 } else { 0 };
                    dst.set(res, &mut self.memory, self.relative_base)?;

                    self.ip += 4;
                }
                Opcode::Equals(op1, op2, dst) => {
                    let val1 = op1.get(&mut self.memory, self.relative_base)?;
                    let val2 = op2.get(&mut self.memory, self.relative_base)?;

                    let res = if val1 == val2 { 1 } else { 0 };
                    dst.set(res, &mut self.memory, self.relative_base)?;

                    self.ip += 4;
                }
                Opcode::AdjustRelBase(offset) => {
                    let offset = offset.get(&mut self.memory, self.relative_base)?;
                    self.relative_base = self.relative_base.wrapping_add(offset as usize);

                    self.ip += 2;
                }
                Opcode::Halt => break Ok(true),
            }
        }
    }

    pub fn run_and_wait(&mut self) -> Result<bool> {
        self.wait_input = true;
        self.exec()
    }

    pub fn run(&mut self) -> Result<()> {
        self.wait_input = false;
        self.exec()?;
        Ok(())
    }

    pub fn get_day02_output(&self) -> Option<i64> {
        self.memory.get(0).copied()
    }

    pub fn get_last_output(&self) -> Option<i64> {
        self.output.last().copied()
    }
}

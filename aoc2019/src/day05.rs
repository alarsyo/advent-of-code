use std::fmt::Write;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day05.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<i64> {
    let mut intcode = Intcode::new(input)?;
    intcode.add_input(1);
    intcode.run()?;
    intcode
        .get_output()
        .ok_or_else(|| err!("intcode gave no output"))
}

fn part2(input: &str) -> Result<i64> {
    let mut intcode = Intcode::new(input)?;
    intcode.add_input(5);
    intcode.run()?;
    intcode
        .get_output()
        .ok_or_else(|| err!("intcode gave no output"))
}

enum Parameter {
    Position(usize),
    Immediate(i64),
}

impl Parameter {
    fn new(mode: i64, val: Option<i64>) -> Result<Self> {
        let val = val.ok_or_else(|| err!("parameter value out of bounds"))?;
        let mode = mode % 10;

        match mode {
            0 => {
                if val < 0 {
                    Err(err!("negative value for position parameter: {}", val))
                } else {
                    let val = val as usize;
                    Ok(Parameter::Position(val))
                }
            }
            1 => Ok(Parameter::Immediate(val)),
            _ => Err(err!("wrong mode for parameter: {}", mode)),
        }
    }

    fn get(&self, memory: &[i64]) -> Option<i64> {
        match self {
            Parameter::Position(address) => memory.get(*address).copied(),
            Parameter::Immediate(value) => Some(*value),
        }
    }

    fn set(&self, memory: &mut [i64], value: i64) -> Result<()> {
        match self {
            Parameter::Position(address) => {
                let cell = memory
                    .get_mut(*address)
                    .ok_or_else(|| err!("memory write out of bounds"))?;
                *cell = value;
                Ok(())
            }
            Parameter::Immediate(_) => Err(err!("cannot write to immediate parameter")),
        }
    }
}

enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

struct Intcode {
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    ip: usize,
}

impl Intcode {
    fn new(input: &str) -> Result<Self> {
        let memory = input
            .trim_end()
            .split(',')
            .map(|x| x.parse().map_err(|e| err!("couldn't parse int: {}", e)))
            .collect::<Result<Vec<i64>>>()?;

        Ok(Intcode {
            memory,
            input: Vec::new(),
            output: Vec::new(),
            ip: 0,
        })
    }

    fn add_input(&mut self, value: i64) {
        self.input.push(value);
    }

    fn get_opcode(&self) -> Result<Opcode> {
        let instruction = self.memory[self.ip];

        let opcode = instruction % 100;
        match opcode {
            1 => {
                let op1 = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let op2 =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;
                let dst =
                    Parameter::new(instruction / 10000, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(err!("add: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Add(op1, op2, dst))
                }
            }
            2 => {
                let op1 = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let op2 =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;
                let dst =
                    Parameter::new(instruction / 10000, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(err!("multiply: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Multiply(op1, op2, dst))
                }
            }
            3 => {
                let dst = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(err!("input: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Input(dst))
                }
            }
            4 => {
                let op = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;

                Ok(Opcode::Output(op))
            }
            5 => {
                let test =
                    Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let dst =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;

                Ok(Opcode::JumpTrue(test, dst))
            }
            6 => {
                let test =
                    Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let dst =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;

                Ok(Opcode::JumpFalse(test, dst))
            }
            7 => {
                let op1 = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let op2 =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;
                let dst =
                    Parameter::new(instruction / 10000, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(err!("less than: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::LessThan(op1, op2, dst))
                }
            }
            8 => {
                let op1 = Parameter::new(instruction / 100, self.memory.get(self.ip + 1).copied())?;
                let op2 =
                    Parameter::new(instruction / 1000, self.memory.get(self.ip + 2).copied())?;
                let dst =
                    Parameter::new(instruction / 10000, self.memory.get(self.ip + 3).copied())?;

                if let Parameter::Immediate(_) = dst {
                    Err(err!("equals: destination parameter can't be immediate"))
                } else {
                    Ok(Opcode::Equals(op1, op2, dst))
                }
            }
            99 => Ok(Opcode::Halt),
            _ => Err(err!("unknown opcode: {}", opcode)),
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            if self.ip >= self.memory.len() {
                return Err(err!("reached end of program without halting"));
            }

            let opcode = self.get_opcode()?;

            match opcode {
                Opcode::Add(op1, op2, dst) => {
                    let val1 = op1
                        .get(&self.memory)
                        .ok_or_else(|| err!("add: op1 out of bounds"))?;
                    let val2 = op2
                        .get(&self.memory)
                        .ok_or_else(|| err!("add: op1 out of bounds"))?;

                    dst.set(&mut self.memory, val1 + val2)?;

                    self.ip += 4;
                }
                Opcode::Multiply(op1, op2, dst) => {
                    let val1 = op1
                        .get(&self.memory)
                        .ok_or_else(|| err!("add: op1 out of bounds"))?;
                    let val2 = op2
                        .get(&self.memory)
                        .ok_or_else(|| err!("add: op1 out of bounds"))?;

                    dst.set(&mut self.memory, val1 * val2)?;

                    self.ip += 4;
                }
                Opcode::Input(dst) => {
                    let input = self
                        .input
                        .pop()
                        .ok_or_else(|| err!("tried to read input but it was empty"))?;
                    dst.set(&mut self.memory, input)?;

                    self.ip += 2;
                }
                Opcode::Output(op) => {
                    let val = op
                        .get(&self.memory)
                        .ok_or_else(|| err!("output: op out of bounds"))?;
                    self.output.push(val);

                    self.ip += 2;
                }
                Opcode::JumpTrue(test, dst) => {
                    let val = test
                        .get(&self.memory)
                        .ok_or_else(|| err!("jump true: op out of bounds"))?;
                    let dst = dst
                        .get(&self.memory)
                        .filter(|dst| *dst >= 0)
                        .ok_or_else(|| err!("jump true: op out of bounds"))?;

                    if val != 0 {
                        self.ip = dst as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Opcode::JumpFalse(test, dst) => {
                    let val = test
                        .get(&self.memory)
                        .ok_or_else(|| err!("jump false: op out of bounds"))?;
                    let dst = dst
                        .get(&self.memory)
                        .filter(|dst| *dst >= 0)
                        .ok_or_else(|| err!("jump false: op out of bounds"))?;

                    if val == 0 {
                        self.ip = dst as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Opcode::LessThan(op1, op2, dst) => {
                    let val1 = op1
                        .get(&self.memory)
                        .ok_or_else(|| err!("less than: op1 out of bounds"))?;
                    let val2 = op2
                        .get(&self.memory)
                        .ok_or_else(|| err!("less than: op1 out of bounds"))?;

                    let res = if val1 < val2 { 1 } else { 0 };
                    dst.set(&mut self.memory, res)?;

                    self.ip += 4;
                }
                Opcode::Equals(op1, op2, dst) => {
                    let val1 = op1
                        .get(&self.memory)
                        .ok_or_else(|| err!("equals: op1 out of bounds"))?;
                    let val2 = op2
                        .get(&self.memory)
                        .ok_or_else(|| err!("equals: op1 out of bounds"))?;

                    let res = if val1 == val2 { 1 } else { 0 };
                    dst.set(&mut self.memory, res)?;

                    self.ip += 4;
                }
                Opcode::Halt => break Ok(()),
            }
        }
    }

    fn get_output(&self) -> Option<i64> {
        for (i, out) in self.output.iter().enumerate() {
            if i < self.output.len() - 1 {
                assert_eq!(*out, 0);
            } else {
                return Some(*out);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 16225258);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 2808771);
    }
}

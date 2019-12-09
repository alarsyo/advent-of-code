use aoc::err;
use aoc::Result;

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl Parameter {
    pub fn new(mode: i64, val: Option<i64>) -> Result<Self> {
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
            2 => Ok(Parameter::Relative(val)),
            _ => Err(err!("wrong mode for parameter: {}", mode)),
        }
    }

    pub fn get(&self, memory: &mut Vec<i64>, relative_base: usize) -> Result<i64> {
        match self {
            Parameter::Position(address) => {
                let cell = memory.get(*address);

                match cell {
                    Some(val) => Ok(*val),
                    // resize memory if tried to read beyond current memory size
                    None => {
                        memory.resize_with(*address + 1, Default::default);
                        Ok(memory[*address])
                    }
                }
            }
            Parameter::Immediate(value) => Ok(*value),
            Parameter::Relative(offset) => {
                let address = relative_base.wrapping_add(*offset as usize);

                let cell = memory.get(address);

                match cell {
                    Some(val) => Ok(*val),
                    // resize memory if tried to read beyond current memory size
                    None => {
                        memory.resize_with(address + 1, Default::default);
                        Ok(memory[address])
                    }
                }
            }
        }
    }

    pub fn set(&self, value: i64, memory: &mut Vec<i64>, relative_base: usize) -> Result<()> {
        match self {
            Parameter::Position(address) => {
                let cell = memory.get_mut(*address);

                match cell {
                    Some(cell) => *cell = value,
                    // resize memory if tried to write beyond current memory size
                    None => {
                        memory.resize_with(*address + 1, Default::default);
                        memory[*address] = value;
                    }
                }
                Ok(())
            }
            Parameter::Immediate(_) => Err(err!("cannot write to immediate parameter")),
            Parameter::Relative(offset) => {
                let address = relative_base.wrapping_add(*offset as usize);
                let cell = memory.get_mut(address);

                match cell {
                    Some(cell) => *cell = value,
                    // resize memory if tried to write beyond current memory size
                    None => {
                        memory.resize_with(address + 1, Default::default);
                        memory[address] = value;
                    }
                }
                Ok(())
            }
        }
    }
}

use aoc::err;
use aoc::Result;

pub enum Parameter {
    Position(usize),
    Immediate(i64),
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
            _ => Err(err!("wrong mode for parameter: {}", mode)),
        }
    }

    pub fn get(&self, memory: &[i64]) -> Result<i64> {
        match self {
            Parameter::Position(address) => match memory.get(*address) {
                Some(val) => Ok(*val),
                None => Err(err!("read out of bounds at address {}", address)),
            },
            Parameter::Immediate(value) => Ok(*value),
        }
    }

    pub fn set(&self, memory: &mut [i64], value: i64) -> Result<()> {
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

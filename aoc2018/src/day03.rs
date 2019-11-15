use std::collections::HashMap;
use std::fmt;

use super::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
}

#[derive(Debug)]
struct ParseError {
    line: String,
}

impl ParseError {
    fn new(line: &str) -> Self {
        ParseError {
            line: line.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.line
    }
}

fn parse(line: &str) -> Option<(usize, usize, usize, usize)> {
    // remove '#XXX @ ' start of line
    let line = &line[(line.find(" @ ")? + 3)..];

    // parse 'x,y: NNxNN
    let comma = line.find(',')?;
    let colon = line.find(':')?;
    let x = line[..comma].parse().ok()?;
    let y = line[(comma + 1)..colon].parse().ok()?;

    // reduce line to 'NNxNN'
    let line = &line[(colon + 2)..];

    let sep = line.find('x')?;
    let width = line[..sep].parse().ok()?;
    let height = line[(sep + 1)..].parse().ok()?;

    Some((x, y, width, height))
}

fn part1(input: &str) -> Result<u64> {
    let mut res = 0;
    let mut map: HashMap<(usize, usize), u64> = HashMap::default();

    for line in input.lines() {
        let (x, y, width, height) = parse(line).ok_or(ParseError::new(line))?;

        for i in 0..width {
            for j in 0..height {
                let x = x + i;
                let y = y + j;

                // add tissue patch at coordinates (x, y)
                let entry = map.entry((x, y)).or_default();
                *entry += 1;

                // count overlap the first time only
                if *entry == 2 {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";
        assert_eq!(part1(input).unwrap(), 4);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 114946);
    }
}

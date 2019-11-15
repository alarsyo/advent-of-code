use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use super::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

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

struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    id: usize,
}

/// Parses a claim from a line
///
/// Fails if the line is badly formatted. The expected format is:
///
/// ```text
/// #ID @ X,Y: WxH
/// ```
fn parse(line: &str) -> Option<Claim> {
    // skip '#' in line
    let line = &line[1..];

    // find ' @ ' separator
    let at = line.find(" @ ")?;
    let id = line[..at].parse().ok()?;
    let line = &line[(at + 3)..];

    // parse 'X,Y: WxH
    let comma = line.find(',')?;
    let colon = line.find(':')?;
    let x = line[..comma].parse().ok()?;
    let y = line[(comma + 1)..colon].parse().ok()?;

    // reduce line to 'WxH'
    let line = &line[(colon + 2)..];

    let sep = line.find('x')?;
    let width = line[..sep].parse().ok()?;
    let height = line[(sep + 1)..].parse().ok()?;

    Some(Claim {
        x,
        y,
        width,
        height,
        id,
    })
}

fn part1(input: &str) -> Result<u64> {
    let mut res = 0;
    let mut map: HashMap<(usize, usize), u64> = HashMap::default();

    for line in input.lines() {
        let claim = parse(line).ok_or(ParseError::new(line))?;

        for i in 0..claim.width {
            for j in 0..claim.height {
                let x = claim.x + i;
                let y = claim.y + j;

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

fn part2(input: &str) -> Result<usize> {
    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::default();
    let mut set = HashSet::new();

    for line in input.lines() {
        let claim = parse(line).ok_or(ParseError::new(line))?;
        set.insert(claim.id);

        for i in 0..claim.width {
            for j in 0..claim.height {
                let x = claim.x + i;
                let y = claim.y + j;

                // add tissue patch at coordinates (x, y)
                let entry = map.entry((x, y)).or_default();
                entry.push(claim.id);

                // if overlap, remove claims from possible solutions
                if entry.len() > 1 {
                    for id in entry {
                        set.remove(id);
                    }
                }
            }
        }
    }

    assert!(!set.is_empty());
    Ok(set.into_iter().next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 4);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 114946);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 3);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 877);
    }
}

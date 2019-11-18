use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use super::err;
use super::Result;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    id: usize,
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    /// Parses a claim from a line
    ///
    /// Fails if the line is badly formatted. The expected format is:
    ///
    /// ```text
    /// #ID @ X,Y: WxH
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        // skip '#' in line
        let s = &s[1..];

        // find ' @ ' separator
        let at = s
            .find(" @ ")
            .ok_or_else(|| err!("` @ ` delimiter not found"))?;
        let id = s[..at].parse()?;
        let s = &s[(at + 3)..];

        // parse 'X,Y: WxH
        let comma = s.find(',').ok_or_else(|| err!("`,` delimiter not found"))?;
        let colon = s.find(':').ok_or_else(|| err!("`:` delimiter not found"))?;
        let x = s[..comma].parse()?;
        let y = s[(comma + 1)..colon].parse()?;

        // reduce line to 'WxH'
        let s = &s[(colon + 2)..];

        let sep = s.find('x').ok_or_else(|| err!("`x` delimiter not found"))?;
        let width = s[..sep].parse()?;
        let height = s[(sep + 1)..].parse()?;

        Ok(Claim {
            x,
            y,
            width,
            height,
            id,
        })
    }
}

fn part1(input: &str) -> Result<u64> {
    let mut res = 0;
    let mut map: HashMap<(usize, usize), u64> = HashMap::new();

    for line in input.lines() {
        let claim: Claim = line
            .parse()
            .or_else(|e| Err(err!("couldn't parse line: `{}`, {}", line, e)))?;

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
    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut set = HashSet::new();

    for line in input.lines() {
        let claim: Claim = line
            .parse()
            .or_else(|e| Err(err!("couldn't parse line: `{}`, {}", line, e)))?;
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

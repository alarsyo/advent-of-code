use md5::{Digest, Md5};

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u64> {
    let input = input.trim_end();
    let mut content = String::from(input);

    let mut hasher = Md5::new();

    for i in 0.. {
        content.truncate(input.len());
        content.extend(i.to_string().chars());

        hasher.input(&content);
        let res = hasher.result_reset();
        if &res[..2] == &[0, 0] && res[2] <= 0x0f {
            return Ok(i);
        }
    }

    Err(err!("couldn't find a suitable number"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // takes too long!
    fn part1_provided() {
        assert_eq!(part1("abcdef").unwrap(), 609043);
        assert_eq!(part1("pqrstuv").unwrap(), 1048970);
    }

    #[test]
    #[ignore] // takes too long!
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 282749);
    }
}

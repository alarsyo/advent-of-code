use std::collections::HashSet;
use std::fmt::Write;

use aoc::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<i32> {
    let freq = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    Ok(freq)
}

fn part2(input: &str) -> Result<i32> {
    let mut freqs = HashSet::new();
    let mut freq = 0;
    loop {
        for line in input.lines() {
            if freqs.contains(&freq) {
                return Ok(freq);
            } else {
                freqs.insert(freq);
            }
            freq += line.parse::<i32>()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "+1
-2
+3
+1
";

    const PROVIDED2: &str = "+1
+1
+1
";

    const PROVIDED3: &str = "+1
+1
-2
";

    const PROVIDED4: &str = "-1
-2
-3
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 3);
        assert_eq!(part1(PROVIDED2).unwrap(), 3);
        assert_eq!(part1(PROVIDED3).unwrap(), 0);
        assert_eq!(part1(PROVIDED4).unwrap(), -6);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 427);
    }

    const PROVIDED5: &str = "+1
-1
";

    const PROVIDED6: &str = "+3
+3
+4
-2
-4
";

    const PROVIDED7: &str = "-6
+3
+8
+5
-6
";

    const PROVIDED8: &str = "+7
+7
-2
-7
-4
";

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED1).unwrap(), 2);
        assert_eq!(part2(PROVIDED5).unwrap(), 0);
        assert_eq!(part2(PROVIDED6).unwrap(), 10);
        assert_eq!(part2(PROVIDED7).unwrap(), 5);
        assert_eq!(part2(PROVIDED8).unwrap(), 14);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 341);
    }
}

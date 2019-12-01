use std::collections::HashSet;

use aoc::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
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

    #[test]
    fn part1_provided1() {
        let input = "+1
-2
+3
+1
";

        assert_eq!(part1(input).unwrap(), 3);
    }

    #[test]
    fn part1_provided2() {
        let input = "+1
+1
+1
";

        assert_eq!(part1(input).unwrap(), 3);
    }

    #[test]
    fn part1_provided3() {
        let input = "+1
+1
-2
";

        assert_eq!(part1(input).unwrap(), 0);
    }

    #[test]
    fn part1_provided4() {
        let input = "-1
-2
-3
";

        assert_eq!(part1(input).unwrap(), -6);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 427);
    }

    #[test]
    fn part2_provided1() {
        let input = "+1
-2
+3
+1
";

        assert_eq!(part2(input).unwrap(), 2);
    }

    #[test]
    fn part2_provided2() {
        let input = "+1
-1
";

        assert_eq!(part2(input).unwrap(), 0);
    }

    #[test]
    fn part2_provided3() {
        let input = "+3
+3
+4
-2
-4
";

        assert_eq!(part2(input).unwrap(), 10);
    }

    #[test]
    fn part2_provided4() {
        let input = "-6
+3
+8
+5
-6
";

        assert_eq!(part2(input).unwrap(), 5);
    }

    #[test]
    fn part2_provided5() {
        let input = "+7
+7
-2
-7
-4
";

        assert_eq!(part2(input).unwrap(), 14);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 341);
    }
}

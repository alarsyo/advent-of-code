use std::collections::HashSet;
use std::env;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let input = fs::read_to_string(
        &args
            .next()
            .expect("Please provide the path to the input file"),
    )?;

    println!("part 1: {}", part1(&input)?);
    println!("part 2: {}", part2(&input)?);

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
                println!("{}", freq);
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
        let input = include_str!("../input/input.txt");

        assert_eq!(part1(input).unwrap(), 427);
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
        let input = include_str!("../input/input.txt");

        assert_eq!(part2(input).unwrap(), 341);
    }
}

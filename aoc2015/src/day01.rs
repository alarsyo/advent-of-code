use super::err;
use super::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i64> {
    Ok(input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut sum = 0;
    let mut res = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => return Err(err!("unidentified character in input: {}", c)),
        };

        if sum < 0 {
            res = i + 1;
            break;
        }
    }

    match res {
        0 => Err(err!("never reached the basement...")),
        _ => Ok(res),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(part1("(())").unwrap(), 0);
        assert_eq!(part1("()()").unwrap(), 0);
        assert_eq!(part1("(((").unwrap(), 3);
        assert_eq!(part1("(()(()(").unwrap(), 3);
        assert_eq!(part1("))(((((").unwrap(), 3);
        assert_eq!(part1("())").unwrap(), -1);
        assert_eq!(part1("))(").unwrap(), -1);
        assert_eq!(part1(")))").unwrap(), -3);
        assert_eq!(part1(")())())").unwrap(), -3);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 74);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(")").unwrap(), 1);
        assert_eq!(part2("()())").unwrap(), 5);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1795);
    }
}

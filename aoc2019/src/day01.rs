use super::Result;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn run() -> Result<()> {
    println!("part 1: {}", part1(INPUT)?);

    Ok(())
}

fn fuel_needed(module_weight: u64) -> u64 {
    (module_weight / 3) - 2
}

fn part1(input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>())
        .map(|w| match w {
            Ok(w) => Ok(fuel_needed(w)),
            Err(e) => Err(Box::from(e)),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided() {
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 654);
        assert_eq!(fuel_needed(100756), 33583);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3268951);
    }
}

use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day10.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let mut jolts = input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|e| err!("couldn't parse joltage: {}", e))
        })
        .collect::<aoc::Result<Vec<u64>>>()?;

    if jolts.is_empty() {
        return Err(err!("input was empty!"));
    }

    // charging outlet can be added
    jolts.insert(0, 0);

    jolts.sort_unstable();

    // device is rated for max adapter + 3 jolts
    let max_adapter = jolts[jolts.len() - 1];
    let device_rating = max_adapter + 3;
    jolts.push(device_rating);

    let mut differences: [usize; 4] = [0; 4];

    for (prev, next) in jolts.iter().zip(jolts.iter().skip(1)) {
        let difference = (next - prev) as usize;
        differences[difference] += 1;
    }

    Ok(differences[1] * differences[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = include_str!("../input/day10_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day10_provided2.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 7 * 5);
        assert_eq!(part1(PROVIDED2).unwrap(), 22 * 10);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 2112);
    }
}

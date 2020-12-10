use std::collections::HashMap;
use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day10.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn find_possibilities(jolts: &[u64], possibilities: &mut HashMap<u64, usize>) -> usize {
    if let Some(&count) = possibilities.get(&jolts[0]) {
        return count;
    }

    if jolts.len() == 1 {
        return 1;
    }

    let curr = jolts[0];

    let possibilities_from_here = jolts
        .iter()
        .copied()
        // we need the index to step in the jolt slice
        .enumerate()
        // skip the current adapter one
        .skip(1)
        // its 3 successors can possibly be removed
        .take(3)
        // remove adapter if the jolt difference is too high
        .filter(|(_, jolt)| jolt - curr <= 3)
        // count each possible next adapter and all its possibilities
        .map(|(idx, _)| find_possibilities(&jolts[idx..], possibilities))
        .sum();

    possibilities.insert(curr, possibilities_from_here);

    possibilities_from_here
}

fn part2(input: &str) -> aoc::Result<usize> {
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

    let mut possibilities = HashMap::new();

    Ok(find_possibilities(&jolts, &mut possibilities))
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

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED1).unwrap(), 8);
        assert_eq!(part2(PROVIDED2).unwrap(), 19208);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 3022415986688);
    }
}

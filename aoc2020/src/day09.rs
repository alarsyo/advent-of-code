use std::fmt::Write;

use aoc::err;

const INPUT: &str = include_str!("../input/day09.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn find_pair_sum(data: &[u64], total: u64) -> Option<(u64, u64)> {
    // on huge entries using a set like in day 01 would be faster, but here it's actually slower,
    // probably due to relatively small input size
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if data[i] + data[j] == total {
                return Some((data[i], data[j]));
            }
        }
    }

    None
}

fn find_outlier(numbers: &[u64], preamble_size: usize) -> aoc::Result<(u64, usize)> {
    // start checking numbers after the preamble only
    for i in preamble_size..numbers.len() {
        let preamble = &numbers[(i - preamble_size)..i];
        let curr = numbers[i];

        match find_pair_sum(preamble, curr) {
            Some(_) => continue,
            None => return Ok((curr, i)),
        }
    }

    Err(err!("couldn't find number with that property"))
}

fn part1(input: &str) -> aoc::Result<u64> {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| err!("couldn't parse number: {}", e))?;

    let (solution, _) = find_outlier(&numbers, 25)?;

    Ok(solution)
}

fn find_contiguous_range(numbers: &[u64], total: u64) -> aoc::Result<(u64, u64)> {
    // compute cumulated sums for the whole range
    let (sums, _) = numbers.iter().fold((vec![0], 0), |(mut vec, acc), n| {
        let acc = acc + n;
        vec.push(acc);

        (vec, acc)
    });

    for i in 0..sums.len() {
        for j in (i + 1)..sums.len() {
            if sums[j] - sums[i] == total {
                let min = numbers[i..j].iter().min().unwrap();
                let max = numbers[i..j].iter().max().unwrap();

                return Ok((*min, *max));
            }
        }
    }

    Err(err!("couldn't find number with that property"))
}

fn part2(input: &str) -> aoc::Result<u64> {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| err!("couldn't parse number: {}", e))?;

    let (outlier, idx) = find_outlier(&numbers, 25)?;

    let (min, max) = find_contiguous_range(&numbers[..idx], outlier)?;

    Ok(min + max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day09_provided.txt");

    #[test]
    fn part1_provided() {
        let numbers = PROVIDED
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        assert_eq!(find_outlier(&numbers, 5).unwrap(), (127, 14));
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 248131121);
    }

    #[test]
    fn part2_provided() {
        let numbers = PROVIDED
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let (outlier, idx) = find_outlier(&numbers, 5).unwrap();

        let (min, max) = find_contiguous_range(&numbers[..idx], outlier).unwrap();

        assert_eq!(min, 15);
        assert_eq!(max, 47);
        assert_eq!(min + max, 62);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 31580383);
    }
}

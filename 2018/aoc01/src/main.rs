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
            freq += line.parse::<i32>()?;
            if freqs.contains(&freq) {
                println!("{}", freq);
                return Ok(freq);
            } else {
                freqs.insert(freq);
            }
        }
    }
}

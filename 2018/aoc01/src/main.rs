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

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let freq = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{}", freq);
}

fn part2(input: &str) {
    let mut freqs = HashSet::new();
    let mut freq = 0;
    loop {
        for line in input.lines() {
            freq += line.parse::<i32>().unwrap();
            if freqs.contains(&freq) {
                println!("{}", freq);
                return;
            } else {
                freqs.insert(freq);
            }
        }
    }
}

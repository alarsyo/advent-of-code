use std::collections::HashMap;
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

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut twice = 0;
    let mut thrice = 0;

    for line in input.lines() {
        let mut seen: HashMap<char, u32> = HashMap::new();
        for c in line.chars() {
            *seen.entry(c).or_default() += 1;
        }

        if seen.values().any(|x| *x == 2) {
            twice += 1;
        }

        if seen.values().any(|x| *x == 3) {
            thrice += 1;
        }
    }

    println!("{}", twice * thrice);

    Ok(())
}

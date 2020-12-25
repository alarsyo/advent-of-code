use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day25.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

const PUB_KEY_SUBJECT: u64 = 7;

// - Set the value to itself multiplied by the subject number.
// - Set the value to the remainder after dividing the value by 20201227.
fn transform(mut value: u64, subject: u64) -> u64 {
    value *= subject;

    value % 20201227
}

fn find_loop_size(pub_key: u64) -> usize {
    let mut value = 1;
    for i in 1.. {
        value = transform(value, PUB_KEY_SUBJECT);

        if value == pub_key {
            return i;
        }
    }

    unreachable!()
}

fn part1(input: &str) -> Result<u64> {
    let mut lines = input.lines();

    let card_pub_key: u64 = lines
        .next()
        .context("no public key found for card")?
        .parse()?;
    let door_pub_key: u64 = lines
        .next()
        .context("no public key found for door")?
        .parse()?;

    let card_loop_size = find_loop_size(card_pub_key);
    let door_loop_size = find_loop_size(door_pub_key);

    let (loop_size, pub_key) = if card_loop_size < door_loop_size {
        (card_loop_size, door_pub_key)
    } else {
        (door_loop_size, card_pub_key)
    };

    let mut encryption_key = 1;
    for _ in 0..loop_size {
        encryption_key = transform(encryption_key, pub_key);
    }

    Ok(encryption_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day25_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
        assert_eq!(part1(PROVIDED).unwrap(), 14897079);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 3015200);
    }
}

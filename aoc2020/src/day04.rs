use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn get_passports(input: &str) -> aoc::Result<Vec<Passport>> {
    let mut passports: Vec<Passport> = Vec::new();

    let mut passport = String::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(passport.parse()?);
            passport.clear();
        } else {
            passport.push('\n');
            passport.push_str(line);
        }
    }

    if !passport.is_empty() {
        passports.push(passport.parse()?);
    }

    Ok(passports)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let passports = get_passports(input)?;

    Ok(passports.iter().filter(|p| p.has_valid_fields()).count())
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    #[allow(dead_code)]
    cid: Option<String>,
}

impl Passport {
    fn has_valid_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl FromStr for Passport {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let mut fields: HashMap<&str, String> = s
            .split_whitespace()
            .map(|f| {
                let mut it = f.split(':');

                let key = it.next().unwrap();
                let value = it.next().unwrap().to_string();

                (key, value)
            })
            .collect();

        Ok(Passport {
            byr: fields.remove("byr"),
            iyr: fields.remove("iyr"),
            eyr: fields.remove("eyr"),
            hgt: fields.remove("hgt"),
            hcl: fields.remove("hcl"),
            ecl: fields.remove("ecl"),
            pid: fields.remove("pid"),
            cid: fields.remove("cid"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED: &'static str = include_str!("../input/day04_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 2);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 192);
    }
}

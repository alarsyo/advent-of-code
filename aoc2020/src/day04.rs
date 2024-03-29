use std::collections::HashMap;
use std::fmt::Write;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn get_passports(input: &str) -> Result<Vec<Passport>> {
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

fn part1(input: &str) -> Result<usize> {
    let passports = get_passports(input)?;

    Ok(passports.iter().filter(|p| p.is_complete()).count())
}

fn part2(input: &str) -> Result<usize> {
    let passports = get_passports(input)?;

    Ok(passports
        .into_iter()
        .filter_map(Passport::complete)
        .filter(CompletePassport::is_valid)
        .count())
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
    fn is_complete(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn complete(mut self) -> Option<CompletePassport> {
        Some(CompletePassport {
            byr: self.byr.take()?,
            iyr: self.iyr.take()?,
            eyr: self.eyr.take()?,
            hgt: self.hgt.take()?,
            hcl: self.hcl.take()?,
            ecl: self.ecl.take()?,
            pid: self.pid.take()?,
        })
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
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

struct CompletePassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl CompletePassport {
    fn is_valid(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }

    fn byr_valid(&self) -> bool {
        is_number_in_range(&self.byr, 1920..=2002)
    }

    fn iyr_valid(&self) -> bool {
        is_number_in_range(&self.iyr, 2010..=2020)
    }

    fn eyr_valid(&self) -> bool {
        is_number_in_range(&self.eyr, 2020..=2030)
    }

    fn hgt_valid(&self) -> bool {
        if let Some(num) = self.hgt.strip_suffix("in") {
            is_number_in_range(num, 59..=76)
        } else if let Some(num) = self.hgt.strip_suffix("cm") {
            is_number_in_range(num, 150..=193)
        } else {
            false
        }
    }

    fn hcl_valid(&self) -> bool {
        if let Some(rest) = self.hcl.strip_prefix('#') {
            rest.chars().filter(|c| !c.is_ascii_hexdigit()).count() == 0
        } else {
            false
        }
    }

    fn ecl_valid(&self) -> bool {
        self.ecl == "amb"
            || self.ecl == "blu"
            || self.ecl == "brn"
            || self.ecl == "gry"
            || self.ecl == "grn"
            || self.ecl == "hzl"
            || self.ecl == "oth"
    }

    fn pid_valid(&self) -> bool {
        self.pid.chars().filter(|c| !c.is_ascii_digit()).count() == 0 && self.pid.len() == 9
    }
}

fn is_number_in_range(s: &str, range: RangeInclusive<i64>) -> bool {
    match s.parse() {
        Ok(res) => range.contains(&res),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = include_str!("../input/day04_provided1.txt");
    const PROVIDED2: &str = include_str!("../input/day04_provided2.txt");
    const PROVIDED3: &str = include_str!("../input/day04_provided3.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1).unwrap(), 2);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 192);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED2).unwrap(), 0);
        assert_eq!(part2(PROVIDED3).unwrap(), 4);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 101);
    }
}

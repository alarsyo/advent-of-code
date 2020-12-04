use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day04.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

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

fn part2(input: &str) -> aoc::Result<usize> {
    let passports = get_passports(input)?;

    Ok(passports.iter().filter(|p| p.is_valid()).count())
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
        let byr = match &self.byr {
            Some(s) => s,
            None => return false,
        };

        if let Ok(res) = byr.parse::<i64>() {
            res >= 1920 && res <= 2002
        } else {
            false
        }
    }

    fn iyr_valid(&self) -> bool {
        let iyr = match &self.iyr {
            Some(s) => s,
            None => return false,
        };

        if let Ok(res) = iyr.parse::<i64>() {
            res >= 2010 && res <= 2020
        } else {
            false
        }
    }

    fn eyr_valid(&self) -> bool {
        let eyr = match &self.eyr {
            Some(s) => s,
            None => return false,
        };

        if let Ok(res) = eyr.parse::<i64>() {
            res >= 2020 && res <= 2030
        } else {
            false
        }
    }

    fn hgt_valid(&self) -> bool {
        let hgt = match &self.hgt {
            Some(s) => s,
            None => return false,
        };

        if let Some(num) = hgt.strip_suffix("in") {
            if let Ok(res) = num.parse::<i64>() {
                res >= 59 && res <= 76
            } else {
                false
            }
        } else if let Some(num) = hgt.strip_suffix("cm") {
            if let Ok(res) = num.parse::<i64>() {
                res >= 150 && res <= 193
            } else {
                false
            }
        } else {
            false
        }
    }

    fn hcl_valid(&self) -> bool {
        let hcl = match &self.hcl {
            Some(s) => s,
            None => return false,
        };

        if let Some(rest) = hcl.strip_prefix("#") {
            rest.chars().filter(|c| !c.is_ascii_hexdigit()).count() == 0
        } else {
            false
        }
    }

    fn ecl_valid(&self) -> bool {
        let ecl = match &self.ecl {
            Some(s) => s,
            None => return false,
        };

        ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth"
    }

    fn pid_valid(&self) -> bool {
        let pid = match &self.pid {
            Some(s) => s,
            None => return false,
        };

        pid.chars().filter(|c| !c.is_ascii_digit()).count() == 0 && pid.len() == 9
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

    static PROVIDED1: &'static str = include_str!("../input/day04_provided1.txt");
    static PROVIDED2: &'static str = include_str!("../input/day04_provided2.txt");
    static PROVIDED3: &'static str = include_str!("../input/day04_provided3.txt");

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

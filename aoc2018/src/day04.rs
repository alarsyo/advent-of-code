use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Result};

const INPUT: &str = include_str!("../input/day04.txt");

fn sorted_lines(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort_unstable();
    lines.join("\n")
}

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(&sorted_lines(INPUT))?)?;

    Ok(res)
}

#[derive(Debug)]
enum Event {
    ShiftChange(u64),
    FallAsleep,
    WakeUp,
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.find("wakes up").is_some() {
            Ok(Event::WakeUp)
        } else if s.find("falls asleep").is_some() {
            Ok(Event::FallAsleep)
        } else if s.find("begins shift").is_some() {
            let pound = s.find('#').context("`#` not found")?;
            let s = &s[(pound + 1)..];
            let space = s.find(' ').context("` ` not found after `#`")?;
            let id = s[..space].parse()?;
            Ok(Event::ShiftChange(id))
        } else {
            Err(anyhow!("unknown event type"))
        }
    }
}

#[derive(Debug)]
struct Date {
    year: u32,
    month: u8,
    day: u8,

    hour: u8,
    minute: u8,
}

impl FromStr for Date {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let l_bracket = s.find('[').context("`[` not found")?;
        let s = &s[(l_bracket + 1)..];
        let dash = s.find('-').context("`-` not found")?;

        let year = s[..dash].parse()?;
        let s = &s[(dash + 1)..];
        let dash = s.find('-').context("`-` not found")?;

        let month = s[..dash].parse()?;
        let s = &s[(dash + 1)..];
        let space = s.find(' ').context("` ` not found")?;

        let day = s[..space].parse()?;
        let s = &s[(space + 1)..];
        let colon = s.find(':').context("`:` not found")?;

        let hour = s[..colon].parse()?;
        let s = &s[(colon + 1)..];
        let r_bracket = s.find(']').context("`]` not found")?;

        let minute = s[..r_bracket].parse()?;

        Ok(Date {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[derive(Debug)]
struct LogEntry {
    date: Date,
    event: Event,
}

impl FromStr for LogEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let date: Date = s.parse().context("couldn't parse date")?;
        let event = s.parse().context("couldn't parse event")?;

        let entry = LogEntry { date, event };

        match entry.event {
            Event::FallAsleep | Event::WakeUp => {
                assert!(entry.date.hour == 0);
            }
            _ => {}
        };

        Ok(entry)
    }
}

fn part1(input: &str) -> Result<u64> {
    let mut guard_id = None;
    let mut map: HashMap<u64, Vec<LogEntry>> = HashMap::new();

    for line in input.lines() {
        let log_entry: LogEntry = line.parse()?;

        if let Event::ShiftChange(id) = log_entry.event {
            guard_id = Some(id);
        }

        match guard_id {
            Some(id) => map.entry(id).or_default().push(log_entry),
            None => bail!("event before first shift"),
        }
    }

    // Fill frequency by minute and by guard
    let mut sleep_freq_per_guard = HashMap::new();

    for (id, log_entries) in map {
        let mut fell_asleep = None;
        let mut sleep_freq: HashMap<u8, u64> = HashMap::new();

        for e in log_entries {
            match e.event {
                Event::ShiftChange(_) => fell_asleep = None, // new day!
                Event::FallAsleep => fell_asleep = Some(e.date.minute),
                Event::WakeUp => match fell_asleep {
                    Some(asleep) => {
                        let awake = e.date.minute;
                        for m in asleep..awake {
                            *sleep_freq.entry(m).or_default() += 1;
                        }
                        fell_asleep = None;
                    }
                    None => bail!("woke up before falling asleep"),
                },
            }
        }

        if fell_asleep.is_some() {
            bail!("fell asleep but never woke up!");
        }

        sleep_freq_per_guard.insert(id, sleep_freq);
    }

    let (heavy_sleeper, _) = sleep_freq_per_guard
        .iter()
        .max_by_key(|(_, freqs)| freqs.values().sum::<u64>())
        .unwrap();

    let heavy_sleeper_freq = &sleep_freq_per_guard[heavy_sleeper];

    let (best_minute, _) = heavy_sleeper_freq
        .iter()
        .max_by_key(|(_, freq)| *freq)
        .unwrap();

    Ok((*best_minute as u64) * heavy_sleeper)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 240);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&sorted_lines(INPUT)).unwrap(), 142515);
    }
}

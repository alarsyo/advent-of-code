use std::fmt::Write;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input/day08.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let entries = input
        .lines()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Entry>>>()?;

    Ok(entries.iter().map(Entry::count_easy_digits_in_output).sum())
}

fn part2(input: &str) -> Result<u64> {
    let entries = input
        .lines()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Entry>>>()?;

    let mut sum = 0;

    for entry in entries {
        let mapping = entry.compute_mapping()?;
        sum += entry.decode_output(&mapping)?;
    }

    Ok(sum)
}

struct Entry<'a> {
    four_digits_output: Vec<&'a str>,
    unique_signals: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn count_easy_digits_in_output(&self) -> usize {
        self.four_digits_output
            .iter()
            .filter_map(|digit| Self::translate_easy_digit(digit))
            .count()
    }

    fn translate_easy_digit(digit: &str) -> Option<u64> {
        match digit.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn compute_mapping(&self) -> Result<Vec<(&'a str, u64)>> {
        let mut mapping = Vec::new();

        // first, let's get the easy digits
        let easy_digits: Vec<(&str, u64)> = self
            .unique_signals
            .iter()
            .filter_map(|signal| Some(*signal).zip(Self::translate_easy_digit(signal)))
            .collect();

        // now we can get `9` for free: it's the signal that uses 6 segments and has 4 in common
        // with the digit `4` (`6` and `0` also use 6 segments, but only have 3 segments in common
        // with `4`).
        let (four, _) = *easy_digits
            .iter()
            .find(|(_, translation)| *translation == 4)
            .context("no signal found for the digit 4!")?;
        let nine = *self
            .unique_signals
            .iter()
            .filter(|signal| signal.len() == 6)
            .find(|signal| four.chars().all(|c| signal.contains(c)))
            .context("couldn't identify any signal corresponding to the digit 9!")?;
        mapping.push((nine, 9));

        // `0` has 2 segments in common with `1`, while `6` only has 1.
        let (one, _) = *easy_digits
            .iter()
            .find(|(_, translation)| *translation == 1)
            .context("no signal found for the digit 1!")?;
        let zero = *self
            .unique_signals
            .iter()
            .filter(|&signal| signal.len() == 6)
            .filter(|&signal| *signal != nine)
            .find(|signal| one.chars().all(|c| signal.contains(c)))
            .context("couldn't identify any signal corresponding to the digit 0!")?;
        mapping.push((zero, 0));

        // `6` is an easy one now, the only other signal with 6 segments, that isn't nine or zero.
        let six = *self
            .unique_signals
            .iter()
            .filter(|signal| signal.len() == 6)
            .find(|&signal| *signal != nine && *signal != zero)
            .context("couldn't identify any signal corresponding to the digit 6!")?;
        mapping.push((six, 6));

        // `2`, `3` and `5` have 5 segments each.
        //
        // `3` has 2 segments in common with `1`.
        let three = *self
            .unique_signals
            .iter()
            .filter(|signal| signal.len() == 5)
            .find(|signal| one.chars().all(|c| signal.contains(c)))
            .context("couldn't identify any signal corresponding to the digit 3!")?;
        mapping.push((three, 3));

        // `5` has all its segments used in `6`, `2` doesn't.
        let five = *self
            .unique_signals
            .iter()
            .filter(|signal| signal.len() == 5)
            .find(|signal| signal.chars().all(|c| six.contains(c)))
            .context("couldn't identify any signal corresponding to the digit 5!")?;
        mapping.push((five, 5));

        // `2` is the last one!
        let two = self
            .unique_signals
            .iter()
            .filter(|signal| signal.len() == 5)
            .find(|&signal| *signal != five && *signal != three)
            .context("couldn't identify any signal corresponding to the digit 2!")?;
        mapping.push((two, 2));

        debug_assert_eq!(mapping.len(), 6);

        Ok(mapping)
    }

    fn decode_output(&self, digit_mapping: &[(&str, u64)]) -> Result<u64> {
        let mut res = 0;
        for digit in &self.four_digits_output {
            // search is kind of ugly, but having to sort everything is probably time consuming as
            // well.
            let digit = if let Some(translation) = Self::translate_easy_digit(digit) {
                translation
            } else {
                *digit_mapping
                    .iter()
                    .find_map(|(signal, translation)| {
                        if digit.len() == signal.len() && digit.chars().all(|c| signal.contains(c))
                        {
                            Some(translation)
                        } else {
                            None
                        }
                    })
                    .with_context(|| format!("couldn't translate digit `{}`", digit))?
            };

            res = (res * 10) + digit;
        }

        Ok(res)
    }
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (signals, output) = s.split_once(" | ").context("couldn't split on ` | `")?;

        Ok(Self {
            four_digits_output: output.trim().split(' ').collect(),
            unique_signals: signals.trim().split(' ').collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day08_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 26);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 488);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 61229);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1040429);
    }
}

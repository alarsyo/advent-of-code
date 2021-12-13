use std::collections::HashSet;
use std::fmt::Write;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("../input/day13.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let (sheet, fold_instructions) = input.split_once("\n\n").context("couldn't split input")?;
    let mut sheet: PaperSheet = sheet.parse()?;
    let fold_instructions: Vec<FoldInstruction> = fold_instructions
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<_>>()?;

    sheet.apply(
        *fold_instructions
            .first()
            .context("had 0 fold instructions")?,
    );

    Ok(sheet.num_points())
}

fn part2(input: &str) -> Result<String> {
    let (sheet, fold_instructions) = input.split_once("\n\n").context("couldn't split input")?;
    let mut sheet: PaperSheet = sheet.parse()?;
    let fold_instructions: Vec<FoldInstruction> = fold_instructions
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<_>>()?;

    for instr in fold_instructions {
        sheet.apply(instr);
    }

    Ok(format!("{}", sheet))
}

struct PaperSheet {
    points: HashSet<(usize, usize)>,
}

impl PaperSheet {
    fn apply(&mut self, instr: FoldInstruction) {
        let mut to_insert = Vec::new();
        let mut to_remove = Vec::new();

        for point in &self.points {
            match instr {
                FoldInstruction::AlongX(x) => {
                    if point.0 <= x {
                        continue;
                    }

                    let new_point = (x - (point.0 - x), point.1);
                    to_insert.push(new_point);
                    to_remove.push(*point);
                }
                FoldInstruction::AlongY(y) => {
                    if point.1 <= y {
                        continue;
                    }

                    let new_point = (point.0, y - (point.1 - y));
                    to_insert.push(new_point);
                    to_remove.push(*point);
                }
            }
        }

        to_remove.iter().for_each(|point| {
            self.points.remove(point);
        });

        to_insert.into_iter().for_each(|point| {
            self.points.insert(point);
        });
    }

    fn num_points(&self) -> usize {
        self.points.len()
    }
}

impl std::fmt::Display for PaperSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // make sure first row appears on its own line
        writeln!(f)?;

        if self.points.is_empty() {
            return writeln!(f, "`empty paper sheet`");
        }

        let (width, _) = self.points.iter().max_by_key(|point| point.0).unwrap();
        let (_, height) = self.points.iter().max_by_key(|point| point.1).unwrap();

        for y in 0..=*height {
            for x in 0..=*width {
                let chr = if self.points.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                f.write_char(chr)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for PaperSheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut points = HashSet::new();

        for l in s.lines() {
            let (x, y) = l
                .trim()
                .split_once(',')
                .context("couldn't parse paper sheet point coordinate")?;
            let (x, y) = (x.parse()?, y.parse()?);

            points.insert((x, y));
        }

        Ok(PaperSheet { points })
    }
}

#[derive(Clone, Copy)]
enum FoldInstruction {
    AlongX(usize),
    AlongY(usize),
}

impl std::str::FromStr for FoldInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (axis, coord) = s
            .split_once('=')
            .context("couldn't parse folding instruction")?;
        let coord = coord.parse()?;

        match axis {
            "fold along x" => Ok(FoldInstruction::AlongX(coord)),
            "fold along y" => Ok(FoldInstruction::AlongY(coord)),
            _ => Err(anyhow!("couldn't parse folding instruction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day13_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 17);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 753);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(
            part2(PROVIDED).unwrap(),
            "
#####
#...#
#...#
#...#
#####
"
        );
    }

    #[test]
    fn part2_real() {
        assert_eq!(
            part2(INPUT).unwrap(),
            "
#..#.####.#....####.#..#...##.###..#..#
#..#....#.#....#....#..#....#.#..#.#.#.
####...#..#....###..####....#.#..#.##..
#..#..#...#....#....#..#....#.###..#.#.
#..#.#....#....#....#..#.#..#.#.#..#.#.
#..#.####.####.####.#..#..##..#..#.#..#
"
        );
    }
}

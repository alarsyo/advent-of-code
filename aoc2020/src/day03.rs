use std::fmt::Write;
use std::iter::FromIterator;
use std::ops::Index;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn run() -> aoc::Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> aoc::Result<usize> {
    let forest = input.parse()?;

    Ok(count_trees(&forest, (3, 1)))
}

fn count_trees(forest: &Forest, (right, down): (usize, usize)) -> usize {
    let vertical_range = (0..forest.height()).step_by(down);
    let horizontal_range = (0..).step_by(right);

    vertical_range.zip(horizontal_range).filter(|(i, j)| forest[*i][*j]).count()
}

#[derive(Debug)]
struct ForestLine {
    trees: Vec<bool>,
}

impl Index<usize> for ForestLine {
    type Output = bool;

    fn index(&self, i: usize) -> &Self::Output {
        &self.trees[i % self.trees.len()]
    }
}

impl FromIterator<bool> for ForestLine {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Self {
            trees: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<ForestLine>,
}

impl Forest {
    fn height(&self) -> usize {
        self.trees.len()
    }
}

impl Index<usize> for Forest {
    type Output = ForestLine;

    fn index(&self, i: usize) -> &Self::Output {
        if i >= self.height() {
            panic!("forest is of size {}, tried to access {}", self.height(), i);
        }

        &self.trees[i]
    }
}

impl FromStr for Forest {
    type Err = aoc::Error;

    fn from_str(s: &str) -> aoc::Result<Self> {
        let trees = s
            .lines()
            .map(|line| line.chars().map(|c| matches!(c, '#')).collect())
            .collect();

        Ok(Self { trees })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROVIDED: &'static str = include_str!("../input/day03_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 7);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 242);
    }
}

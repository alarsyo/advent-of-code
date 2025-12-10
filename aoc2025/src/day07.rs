use anyhow::{Result, anyhow, bail};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::Write,
    str::FromStr,
};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

#[derive(Debug, Copy, Clone)]
enum DiagramCell {
    Source,
    EmptySpace,
    Splitter,
}

impl FromStr for DiagramCell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "S" => Ok(DiagramCell::Source),
            "." => Ok(DiagramCell::EmptySpace),
            "^" => Ok(DiagramCell::Splitter),
            _ => Err(anyhow!(
                "cannot parse unexpected string `{}' as DiagramCell",
                s
            )),
        }
    }
}

#[derive(Debug, Clone)]
struct ManifoldDiagram {
    grid: Vec<Vec<DiagramCell>>,
    source_coords: (usize, usize),
}

impl FromStr for ManifoldDiagram {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid = Vec::new();
        let mut source_coords = None;
        for (line_idx, line) in s.lines().enumerate() {
            let mut grid_line = Vec::new();
            for cell_idx in 0..line.len() {
                let cell = line[cell_idx..=cell_idx].parse()?;
                if let DiagramCell::Source = cell {
                    match source_coords {
                        None => source_coords = Some((cell_idx, line_idx)),
                        Some(_) => bail!("found two different sources in diagram"),
                    }
                }
                grid_line.push(cell);
            }
            grid.push(grid_line);
        }

        source_coords
            .ok_or(anyhow!("couldn't find source in diagram"))
            .map(|source_coords| Self {
                grid,
                source_coords,
            })
    }
}

impl ManifoldDiagram {
    fn count_ray_splits(&self) -> usize {
        let mut ray_sources = vec![self.source_coords];
        // TODO: we use a HashMap<(usize, usize), ()> because a HashSet doesn't have `entry` methods in stable:
        // https://github.com/rust-lang/rust/issues/60896
        let mut used_splitters = HashMap::new();

        let mut count = 0;
        while let Some(ray_source) = ray_sources.pop() {
            let (x, mut y) = ray_source;
            y += 1;
            while y < self.grid.len()
                && let DiagramCell::EmptySpace = self.grid[y][x]
            {
                y += 1;
            }

            if y == self.grid.len() {
                // We've reached the end of the diagram, we can stop computing this ray
                continue;
            }
            // otherwise, we've hit a splitter (unless there are multiple sources, which shouldn't happen)
            if let DiagramCell::Splitter = self.grid[y][x] {
                // make sure we don't exit the grid when splitting, shouldn't happen
                assert!(x > 0 && x < (self.grid[0].len() - 1));

                if let Entry::Vacant(e) = used_splitters.entry((x, y)) {
                    e.insert(());
                } else {
                    continue;
                }
                ray_sources.push((x - 1, y));
                ray_sources.push((x + 1, y));
                count += 1;
            }
        }

        count
    }

    // This is an iterative implementation of the obvious recursive algorithm for Part 2. It's
    // probably unnecessarily complex written this way, it ended up that way because I reused the
    // Part 1 code and felt it was faster to adapt it rather than rewrite the recursive version :)
    fn count_timelines(&self) -> usize {
        let mut ray_sources = vec![self.source_coords];
        let mut timelines_from: HashMap<(usize, usize), usize> = HashMap::new();

        while let Some(ray_source) = ray_sources.pop() {
            let (x, mut y) = ray_source;
            y += 1;
            while y < self.grid.len()
                && let DiagramCell::EmptySpace = self.grid[y][x]
            {
                y += 1;
            }

            if y == self.grid.len() {
                // We've reached the end of the diagram, this ray is a single timeline
                timelines_from.insert(ray_source, 1);
                continue;
            }

            // Otherwise, we've hit a splitter (unless there are multiple sources, which shouldn't happen)
            if let DiagramCell::Splitter = self.grid[y][x] {
                // Make sure we don't exit the grid when splitting, shouldn't happen
                assert!(x > 0 && x < (self.grid[0].len() - 1));
                let (ray_left, ray_right) = ((x - 1, y), (x + 1, y));

                let left = timelines_from.get(&ray_left);
                let right = timelines_from.get(&ray_right);
                match (left, right) {
                    (Some(res_left), Some(res_right)) => {
                        timelines_from.insert(ray_source, res_left + res_right);
                    }
                    (None, Some(_)) => {
                        ray_sources.push(ray_source);
                        ray_sources.push(ray_left);
                    }
                    (Some(_), None) => {
                        ray_sources.push(ray_source);
                        ray_sources.push(ray_right);
                    }
                    (None, None) => {
                        ray_sources.push(ray_source);
                        ray_sources.push(ray_left);
                        ray_sources.push(ray_right);
                    }
                }
            }
        }

        *timelines_from.get(&self.source_coords).unwrap()
    }
}

fn part1(input: &str) -> Result<usize> {
    let diagram = input.parse::<ManifoldDiagram>()?;
    Ok(diagram.count_ray_splits())
}

fn part2(input: &str) -> Result<usize> {
    let diagram = input.parse::<ManifoldDiagram>()?;
    Ok(diagram.count_timelines())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day07_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 21);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 1537);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(PROVIDED).unwrap(), 40);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 18818811755665);
    }
}

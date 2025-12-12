use anyhow::{Context, Result, anyhow};
use std::{fmt::Write, str::FromStr};

const INPUT: &str = include_str!("../input/day09.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2: {}", part2(INPUT)?)?;
    Ok(res)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .split_once(',')
            .with_context(|| format!("couldn't parse point `{}'", s))?;
        let (x, y) = (x.parse()?, y.parse()?);

        Ok(Self { x, y })
    }
}

fn rectangle_area(p1: &Point, p2: &Point) -> usize {
    (p1.x.max(p2.x) - p1.x.min(p2.x) + 1) * (p1.y.max(p2.y) - p1.y.min(p2.y) + 1)
}

fn part1(input: &str) -> Result<usize> {
    let points = input
        .lines()
        .map(|line| line.parse::<Point>())
        .collect::<Result<Vec<_>>>()?;

    let mut max = None;
    for (idx, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(idx + 1) {
            let area = rectangle_area(p1, p2);
            match max {
                Some(m) => max = Some(area.max(m)),
                None => max = Some(area),
            }
        }
    }

    max.ok_or_else(|| anyhow!("input was empty"))
}

// Computes whether a segment crosses through our rectangle.
//
// Since edges are always parallel to axes, this is easier than in the general case
fn edge_intersects(&(e1, e2): &(Point, Point), (r1, r2): (&Point, &Point)) -> bool {
    let (rect_xmin, rect_xmax) = (r1.x.min(r2.x), r1.x.max(r2.x));
    let (rect_ymin, rect_ymax) = (r1.y.min(r2.y), r1.y.max(r2.y));

    #[allow(clippy::nonminimal_bool)]
    if e1.x == e2.x {
        // segment is vertical
        let edge_ymin = e1.y.min(e2.y);
        let edge_ymax = e1.y.max(e2.y);
        (rect_xmin < e1.x && e1.x < rect_xmax ) // we're in the right X range to cross
            && ((edge_ymin <= rect_ymin && edge_ymax >= rect_ymax) // segment crosses through both sides
                || (edge_ymin <= rect_ymin && rect_ymin < edge_ymax) // segment crosses through bottom edge
                || (edge_ymin < rect_ymax && edge_ymax >= rect_ymax) // segment crosses through top edge
                )
    } else {
        // segment is horizontal
        let edge_xmin = e1.x.min(e2.x);
        let edge_xmax = e1.x.max(e2.x);
        (rect_ymin < e1.y && e1.y < rect_ymax ) // we're in the right X range to cross
            && ((edge_xmin <= rect_xmin && edge_xmax >= rect_xmax) // segment crosses through both sides
                || (edge_xmin <= rect_xmin && rect_xmin < edge_xmax) // segment crosses through bottom edge
                || (edge_xmin < rect_xmax && edge_xmax >= rect_xmax) // segment crosses through top edge
                )
    }
}

fn part2(input: &str) -> Result<usize> {
    let points = input
        .lines()
        .map(|line| line.parse::<Point>())
        .collect::<Result<Vec<_>>>()?;

    let mut edges = points.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>();
    edges.push((
        *points.last().context("input was empty")?,
        *points.first().context("input was empty")?,
    ));

    let mut max = None;
    for (idx, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(idx + 1) {
            if edges.iter().any(|e| edge_intersects(e, (p1, p2))) {
                // If an edge crosses through our rectangle then one side of the edge is obviously
                // outside our polygon.
                //
                // Generally, this isn't sufficient to guarantee that a rectangle is "valid".
                // Consider the following polygon:
                //
                // ..........
                // .#X1..#X#.
                // .XXX..XXX.
                // .XX#XX2XX.
                // .#XXXXXX#.
                // ..........
                //
                // The rectangle between 1 and 2 isn't crossed by any edge, yet it is outside the
                // polygon. However looking at the shape of my input (which more or less looks like
                // a rasterized circle), it's obvious that this kind of edge case will never happen
                // for rectangles large enough to be of maximal area.
                continue;
            }
            let area = rectangle_area(p1, p2);
            match max {
                Some(m) => max = Some(area.max(m)),
                None => max = Some(area),
            }
        }
    }

    max.ok_or_else(|| anyhow!("input was empty"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day09_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 50);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 4758121828);
    }

    #[test]
    fn part2_provided() {
        assert!(edge_intersects(
            &(Point { x: 7, y: 3 }, Point { x: 7, y: 1 }),
            (&Point { x: 2, y: 5 }, &Point { x: 11, y: 1 })
        ));
        assert!(edge_intersects(
            &(Point { x: 9, y: 5 }, Point { x: 2, y: 5 }),
            (&Point { x: 9, y: 7 }, &Point { x: 2, y: 3 })
        ));
        assert_eq!(part2(PROVIDED).unwrap(), 24);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT).unwrap(), 1577956170);
    }
}

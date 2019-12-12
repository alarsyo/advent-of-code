use std::error::Error;
use std::fmt::Write;
use std::str::FromStr;

use aoc::err;
use aoc::Result;

const INPUT: &str = include_str!("../input/day12.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT, 1000)?)?;

    Ok(res)
}

fn generate_pairs_rec(res: &mut Vec<(usize, usize)>, tmp: &mut Vec<usize>, todo: usize, n: usize) {
    if tmp.len() == 2 {
        res.push((tmp[0], tmp[1]));
        return;
    }

    for i in todo..n {
        tmp.push(i);
        generate_pairs_rec(res, tmp, i + 1, n);
        tmp.pop();
    }
}

fn generate_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut tmp = Vec::new();

    generate_pairs_rec(&mut res, &mut tmp, 0, n);

    res
}

fn part1(input: &str, steps: usize) -> Result<u64> {
    let mut planets = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Planet>>>()?;

    let pairs = generate_pairs(planets.len());

    for _ in 0..steps {
        // update velocity
        for pair in pairs.iter().copied() {
            let (begin, end) = planets.split_at_mut(pair.1);
            let first = &mut begin[pair.0];
            let second = &mut end[0];

            if first.position.x > second.position.x {
                first.velocity.x -= 1;
                second.velocity.x += 1;
            } else if first.position.x < second.position.x {
                first.velocity.x += 1;
                second.velocity.x -= 1;
            }

            if first.position.y > second.position.y {
                first.velocity.y -= 1;
                second.velocity.y += 1;
            } else if first.position.y < second.position.y {
                first.velocity.y += 1;
                second.velocity.y -= 1;
            }

            if first.position.z > second.position.z {
                first.velocity.z -= 1;
                second.velocity.z += 1;
            } else if first.position.z < second.position.z {
                first.velocity.z += 1;
                second.velocity.z -= 1;
            }
        }

        // update position
        for planet in planets.iter_mut() {
            planet.update_pos();
        }
    }

    Ok(planets.iter().map(|p| p.total_energy()).sum())
}

struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

struct Planet {
    position: Vec3,
    velocity: Vec3,
}

impl Planet {
    fn new(position: Vec3) -> Self {
        Planet {
            position,
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    fn update_pos(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> u64 {
        let pos = &self.position;
        pos.x.abs() as u64 + pos.y.abs() as u64 + pos.z.abs() as u64
    }

    fn kinetic_energy(&self) -> u64 {
        let vel = &self.velocity;
        vel.x.abs() as u64 + vel.y.abs() as u64 + vel.z.abs() as u64
    }

    fn total_energy(&self) -> u64 {
        self.kinetic_energy() * self.potential_energy()
    }
}

impl FromStr for Planet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let x_equals = s
            .find("x=")
            .ok_or_else(|| err!("couldn't find x value for planet: {}", s))?;

        let comma = s
            .find(',')
            .ok_or_else(|| err!("couldn't find comma after x value: {}", s))?;

        let x = s[(x_equals + 2)..comma].parse()?;
        let s = &s[(comma + 1)..];

        let y_equals = s
            .find("y=")
            .ok_or_else(|| err!("couldn't find y value for planet: {}", s))?;

        let comma = s
            .find(',')
            .ok_or_else(|| err!("couldn't find comma after y value: {}", s))?;

        let y = s[(y_equals + 2)..comma].parse()?;
        let s = &s[(comma + 1)..];

        let z_equals = s
            .find("z=")
            .ok_or_else(|| err!("couldn't find z value for planet: {}", s))?;

        let bracket = s
            .find('>')
            .ok_or_else(|| err!("couldn't find bracket after z value: {}", s))?;

        let z = s[(z_equals + 2)..bracket].parse()?;

        Ok(Planet::new(Vec3 { x, y, z }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

    const PROVIDED2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED1, 10).unwrap(), 179);
        assert_eq!(part1(PROVIDED2, 100).unwrap(), 1940);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT, 1000).unwrap(), 14907);
    }
}

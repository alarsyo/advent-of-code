use std::cmp::Ordering;
use std::fmt::Write;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("../input/day12.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    let planets = parse_planets(INPUT)?;

    writeln!(res, "part 1: {}", part1(planets.clone(), 1000)?)?;
    writeln!(res, "part 2: {}", part2(planets)?)?;

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

fn parse_planets(input: &str) -> Result<Vec<Planet>> {
    input.lines().map(str::parse).collect()
}

fn part1(mut planets: Vec<Planet>, steps: usize) -> Result<u64> {
    let pairs = generate_pairs(planets.len());

    for _ in 0..steps {
        // update velocity
        for pair in pairs.iter().copied() {
            let (begin, end) = planets.split_at_mut(pair.1);
            let first = &mut begin[pair.0];
            let second = &mut end[0];

            match first.position.x.cmp(&second.position.x) {
                Ordering::Greater => {
                    first.velocity.x -= 1;
                    second.velocity.x += 1;
                }
                Ordering::Less => {
                    first.velocity.x += 1;
                    second.velocity.x -= 1;
                }
                Ordering::Equal => {}
            }

            match first.position.y.cmp(&second.position.y) {
                Ordering::Greater => {
                    first.velocity.y -= 1;
                    second.velocity.y += 1;
                }
                Ordering::Less => {
                    first.velocity.y += 1;
                    second.velocity.y -= 1;
                }
                Ordering::Equal => {}
            }

            match first.position.z.cmp(&second.position.z) {
                Ordering::Greater => {
                    first.velocity.z -= 1;
                    second.velocity.z += 1;
                }
                Ordering::Less => {
                    first.velocity.z += 1;
                    second.velocity.z -= 1;
                }
                Ordering::Equal => {}
            }
        }

        // update position
        for planet in &mut planets {
            planet.update_pos();
        }
    }

    Ok(planets.iter().map(Planet::total_energy).sum())
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(mut planets: Vec<Planet>) -> Result<usize> {
    let first_state = planets.clone();

    let pairs = generate_pairs(planets.len());

    let mut x_step = None;
    let mut y_step = None;
    let mut z_step = None;

    for i in 1.. {
        // update velocity
        for pair in pairs.iter().copied() {
            let (begin, end) = planets.split_at_mut(pair.1);
            let first = &mut begin[pair.0];
            let second = &mut end[0];

            match first.position.x.cmp(&second.position.x) {
                Ordering::Greater => {
                    first.velocity.x -= 1;
                    second.velocity.x += 1;
                }
                Ordering::Less => {
                    first.velocity.x += 1;
                    second.velocity.x -= 1;
                }
                Ordering::Equal => {}
            }

            match first.position.y.cmp(&second.position.y) {
                Ordering::Greater => {
                    first.velocity.y -= 1;
                    second.velocity.y += 1;
                }
                Ordering::Less => {
                    first.velocity.y += 1;
                    second.velocity.y -= 1;
                }
                Ordering::Equal => {}
            }

            match first.position.z.cmp(&second.position.z) {
                Ordering::Greater => {
                    first.velocity.z -= 1;
                    second.velocity.z += 1;
                }
                Ordering::Less => {
                    first.velocity.z += 1;
                    second.velocity.z -= 1;
                }
                Ordering::Equal => {}
            }
        }

        // update position
        for planet in &mut planets {
            planet.update_pos();
        }

        if x_step.is_none()
            && planets
                .iter()
                .zip(first_state.iter())
                .all(|(a, b)| a.position.x == b.position.x && a.velocity.x == b.velocity.x)
        {
            x_step = Some(i);
        }

        if y_step.is_none()
            && planets
                .iter()
                .zip(first_state.iter())
                .all(|(a, b)| a.position.y == b.position.y && a.velocity.y == b.velocity.y)
        {
            y_step = Some(i);
        }

        if z_step.is_none()
            && planets
                .iter()
                .zip(first_state.iter())
                .all(|(a, b)| a.position.z == b.position.z && a.velocity.z == b.velocity.z)
        {
            z_step = Some(i);
        }

        if let (Some(a), Some(b), Some(c)) = (x_step, y_step, z_step) {
            return Ok(lcm(a, lcm(b, c)));
        }
    }

    Err(anyhow!("planets never reached the same state twice"))
}

#[derive(Clone)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone)]
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let x_equals = s
            .find("x=")
            .with_context(|| format!("couldn't find x value for planet: {}", s))?;

        let comma = s
            .find(',')
            .with_context(|| format!("couldn't find comma after x value: {}", s))?;

        let x = s[(x_equals + 2)..comma].parse()?;
        let s = &s[(comma + 1)..];

        let y_equals = s
            .find("y=")
            .with_context(|| format!("couldn't find y value for planet: {}", s))?;

        let comma = s
            .find(',')
            .with_context(|| format!("couldn't find comma after y value: {}", s))?;

        let y = s[(y_equals + 2)..comma].parse()?;
        let s = &s[(comma + 1)..];

        let z_equals = s
            .find("z=")
            .with_context(|| format!("couldn't find z value for planet: {}", s))?;

        let bracket = s
            .find('>')
            .with_context(|| format!("couldn't find bracket after z value: {}", s))?;

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
        assert_eq!(part1(parse_planets(PROVIDED1).unwrap(), 10).unwrap(), 179);
        assert_eq!(part1(parse_planets(PROVIDED2).unwrap(), 100).unwrap(), 1940);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(parse_planets(INPUT).unwrap(), 1000).unwrap(), 14907);
    }

    #[test]
    fn part2_provided() {
        assert_eq!(part2(parse_planets(PROVIDED1).unwrap()).unwrap(), 2772);
        assert_eq!(
            part2(parse_planets(PROVIDED2).unwrap()).unwrap(),
            4686774924
        );
    }

    #[test]
    fn part2_real() {
        assert_eq!(
            part2(parse_planets(INPUT).unwrap()).unwrap(),
            467_081_194_429_464
        );
    }
}

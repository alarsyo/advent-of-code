use anyhow::{Context, Result, bail};
use std::{collections::HashMap, fmt::Write, hash::Hash, str::FromStr};

const INPUT: &str = include_str!("../input/day08.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);
    writeln!(res, "part 1: {}", part1(INPUT, 1000)?)?;
    Ok(res)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for JunctionBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, rest) = s
            .split_once(',')
            .context("couldn't split JunctionBox coordinates on comma")?;
        let (y, z) = rest
            .split_once(',')
            .context("couldn't split JunctionBox coordinates on second comma")?;

        let x = x.parse()?;
        let y = y.parse()?;
        let z = z.parse()?;

        Ok(Self { x, y, z })
    }
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        ((self.x as f64 - other.x as f64).powi(2)
            + (self.y as f64 - other.y as f64).powi(2)
            + (self.z as f64 - other.z as f64).powi(2))
        .sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnionFindKey(usize);

/// UnionFind implementation that maintains a circular linked list in addition to the usual tree for
/// each disjoint subset, allowing easy iteration on a subset's members.
struct UnionFind<T: Sized + Hash> {
    members_to_keys: HashMap<T, UnionFindKey>,
    parent: Vec<UnionFindKey>,
    #[allow(unused)]
    next: Vec<UnionFindKey>,
    size: Vec<usize>,
}

impl<T: Sized + Hash + Eq> UnionFind<T> {
    fn from(members: Vec<T>) -> Self {
        let keys: Vec<UnionFindKey> = (0..members.len()).map(UnionFindKey).collect();
        Self {
            parent: keys.clone(),
            next: keys.clone(),
            size: (0..members.len()).map(|_| 1).collect(),
            members_to_keys: members.into_iter().zip(keys).collect(),
        }
    }

    fn get_key(&self, member: &T) -> Result<UnionFindKey> {
        self.members_to_keys
            .get(member)
            .context("unknown member")
            .copied()
    }

    fn union(&mut self, left: &T, right: &T) -> Result<UnionFindKey> {
        let left = self.get_key(left)?;
        let right = self.get_key(right)?;
        Ok(self.union_by_key(left, right))
    }

    fn union_by_key(&mut self, left: UnionFindKey, right: UnionFindKey) -> UnionFindKey {
        let mut left_root = self.find_by_key(left);
        let mut right_root = self.find_by_key(right);

        if left_root == right_root {
            return left_root;
        }

        if self.size[left_root.0] < self.size[right_root.0] {
            (left_root, right_root) = (right_root, left_root)
        }

        self.parent[right_root.0] = left_root;
        let new_size = self.size[left_root.0] + self.size[right_root.0];
        self.size[left_root.0] = new_size;

        left_root
    }

    #[allow(unused)]
    fn find(&mut self, member: &T) -> Result<UnionFindKey> {
        let key = self.get_key(member)?;
        Ok(self.find_by_key(key))
    }

    // This could be implemented with &self using interior mutability...
    fn find_by_key(&mut self, key: UnionFindKey) -> UnionFindKey {
        if key != self.parent[key.0] {
            self.parent[key.0] = self.find_by_key(self.parent[key.0]);
        }

        self.parent[key.0]
    }

    fn roots(&self) -> impl Iterator<Item = UnionFindKey> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(idx, &key)| if idx == key.0 { Some(key) } else { None })
    }

    fn get_size(&mut self, key: UnionFindKey) -> usize {
        let root = self.find_by_key(key);
        self.size[root.0]
    }
}

fn part1(input: &str, connections: usize) -> Result<usize> {
    let points = input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut distances = Vec::new();
    for (i, &pi) in points.iter().enumerate() {
        for &pj in points.iter().skip(i + 1) {
            distances.push((pi.distance_to(&pj), (pi, pj)));
        }
    }
    distances.sort_by(|(dist1, _), (dist2, _)| dist1.total_cmp(dist2));
    if connections > distances.len() {
        bail!(
            "There are less possible matchings than the required number of connections: {} matchings",
            distances.len()
        );
    }

    let mut uf = UnionFind::from(points);
    for (_, (p1, p2)) in distances.into_iter().take(connections) {
        uf.union(&p1, &p2)?;
    }

    let roots = uf.roots().collect::<Vec<_>>();
    let mut sizes = roots.iter().map(|r| uf.get_size(*r)).collect::<Vec<_>>();
    sizes.sort_by_key(|v| std::cmp::Reverse(*v));

    Ok(sizes[0] * sizes[1] * sizes[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day08_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED, 10).unwrap(), 40);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT, 1000).unwrap(), 127551);
    }

    //#[test]
    //fn part2_provided() {
    //    assert_eq!(part2(PROVIDED).unwrap(), 40);
    //}

    //#[test]
    //fn part2_real() {
    //    assert_eq!(part2(INPUT).unwrap(), 18818811755665);
    //}
}

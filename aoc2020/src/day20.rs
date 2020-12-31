use std::collections::HashMap;
use std::fmt::Write;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("../input/day20.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128);

    writeln!(res, "part 1: {}", part1(INPUT)?)?;
    writeln!(res, "part 2:\n{}", part2(INPUT)?)?;

    Ok(res)
}

fn part1(input: &str) -> Result<u64> {
    let tiles: Vec<Tile> = input.split("\n\n").map(str::parse).collect::<Result<_>>()?;

    Ok(tiles
        .iter()
        .filter_map(|tile| {
            let count = tile.neighbours(&tiles).len();

            // corners have 2 edges in common
            if count == 2 {
                Some(tile.id)
            } else {
                None
            }
        })
        .product())
}

fn part2(input: &str) -> Result<String> {
    let tiles: Vec<Tile> = input.split("\n\n").map(str::parse).collect::<Result<_>>()?;

    let image = Image::from_tiles(&tiles);

    Ok(format!("{}", image))
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    R90,
    R180,
    R270,
}

/// Represents a transformation of a tile or image.
///
/// Note: we don't need a horizontal and a vertical flip, these result in the same output as a 180
/// degree rotation when combined, so only one is necessary
#[derive(Debug, Clone, Copy)]
struct Transform {
    flip: bool,
    rotation: Option<Rotation>,
}

impl Transform {
    fn new(flip: bool, rotation: Option<Rotation>) -> Self {
        Self { flip, rotation }
    }

    fn all() -> Vec<Transform> {
        vec![
            Transform::new(false, None),
            Transform::new(false, Some(Rotation::R90)),
            Transform::new(false, Some(Rotation::R180)),
            Transform::new(false, Some(Rotation::R270)),
            Transform::new(true, None),
            Transform::new(true, Some(Rotation::R90)),
            Transform::new(true, Some(Rotation::R180)),
            Transform::new(true, Some(Rotation::R270)),
        ]
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            flip: false,
            rotation: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Position {
    Down,
    Left,
    Right,
    Up,
}

impl Position {
    fn opposite(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
        }
    }

    fn ordered() -> [Self; 4] {
        [Self::Down, Self::Left, Self::Right, Self::Up]
    }

    fn apply(&self, (i, j): (i64, i64)) -> (i64, i64) {
        let (mut di, mut dj) = (0, 0);

        match self {
            Self::Down => di = 1,
            Self::Left => dj = -1,
            Self::Right => dj = 1,
            Self::Up => di = -1,
        }

        (i + di, j + dj)
    }
}

const TILE_WIDTH: usize = 10;
const TILE_HEIGHT: usize = 10;

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    cells: [[bool; TILE_WIDTH]; TILE_HEIGHT],
    transform: Transform,
}

type Borders = [Vec<bool>; 4];

impl Tile {
    fn with_transform(&self, transform: Transform) -> Self {
        let mut res = self.clone();
        res.transform = transform;
        res
    }

    /// Returns the tile's 4 borders, according to its current transformation.
    ///
    /// See [`Self::borders_with_transform()`] for more details
    fn borders(&self) -> Borders {
        self.borders_with_transform(self.transform)
    }

    /// Returns the tile's 4 borders, according to the provided transformation.
    ///
    /// Each border is associated with its position on the tile
    fn borders_with_transform(&self, transform: Transform) -> Borders {
        let mut up = Vec::new();
        let mut down = Vec::new();
        for k in 0..TILE_WIDTH {
            up.push(self.get_with_transform(0, k, transform));
            down.push(self.get_with_transform(TILE_HEIGHT - 1, k, transform));
        }

        let mut left = Vec::new();
        let mut right = Vec::new();
        for k in 0..TILE_HEIGHT {
            left.push(self.get_with_transform(k, 0, transform));
            right.push(self.get_with_transform(k, TILE_WIDTH - 1, transform));
        }

        // NOTE: the ordering is important, must use the enum's integer representations as indices
        [down, left, right, up]
    }

    /// Returns the pixel at indices (i, j) in the tile.
    ///
    /// Uses the tile's current `self.transform`.
    fn get(&self, i: usize, j: usize) -> bool {
        self.get_with_transform(i, j, self.transform)
    }

    /// Returns the pixel at indices (i, j) in the tile, using the provided transform.
    fn get_with_transform(&self, mut i: usize, mut j: usize, transform: Transform) -> bool {
        if let Some(rotation) = transform.rotation {
            match rotation {
                Rotation::R90 => {
                    let prev_i = i;
                    i = j;
                    j = (TILE_WIDTH - 1) - prev_i;
                }
                Rotation::R180 => {
                    i = (TILE_HEIGHT - 1) - i;
                    j = (TILE_WIDTH - 1) - j;
                }
                Rotation::R270 => {
                    let prev_j = j;
                    j = i;
                    i = (TILE_HEIGHT - 1) - prev_j;
                }
            }
        }

        if transform.flip {
            i = (TILE_HEIGHT - 1) - i;
        }

        self.cells[i][j]
    }

    /// Returns a list of neighbour tiles, along with the offset where they'd fit compared to the
    /// current tile (depending on which borders match)
    fn neighbours(&self, tiles: &[Tile]) -> Vec<(Position, Tile)> {
        let borders = &self.borders();

        tiles
            .iter()
            .filter(|other| *other != self)
            .flat_map(|other| {
                Transform::all().into_iter().filter_map(move |transform| {
                    let other_borders = other.borders_with_transform(transform);

                    for (bord, pos) in other_borders.iter().zip(Position::ordered().iter()) {
                        let opposite = pos.opposite();

                        if bord == &borders[opposite as usize] {
                            return Some((opposite, other.with_transform(transform)));
                        }
                    }

                    None
                })
            })
            .collect()
    }
}

impl std::cmp::PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::str::FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let title = lines.next().context("couldn't find line with tile ID")?;
        let space = title.find(' ').unwrap();
        let colon = title.find(':').unwrap();
        let id = title[(space + 1)..colon].parse()?;

        let mut cells = [[false; TILE_WIDTH]; TILE_HEIGHT];

        lines
            .enumerate()
            .try_for_each::<_, Result<()>>(|(i, line)| {
                line.chars().enumerate().try_for_each(|(j, c)| {
                    let c = match c {
                        '#' => true,
                        '.' => false,
                        _ => return Err(anyhow!("unknown char `{}` while parsing tile", c)),
                    };

                    cells[i][j] = c;

                    Ok(())
                })?;

                Ok(())
            })?;

        Ok(Tile {
            id,
            cells,
            transform: Transform::default(),
        })
    }
}

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl Image {
    fn from_tiles(tiles: &[Tile]) -> Self {
        let mut todo: Vec<(i64, i64)> = vec![(0, 0)];
        let mut image_positions = HashMap::new();
        image_positions.insert((0, 0), tiles[0].clone());

        // compute each image position depending on its neighbours
        while !todo.is_empty() {
            let pos = todo.pop().unwrap();
            let tile = &image_positions[&pos];

            for (direction, other_tile) in tile.neighbours(tiles) {
                let new_pos = direction.apply(pos);
                #[allow(clippy::map_entry)]
                if !image_positions.contains_key(&new_pos) {
                    image_positions.insert(new_pos, other_tile);
                    todo.push(new_pos);
                }
            }
        }

        let image_positions = image_positions.into_iter().collect::<Vec<_>>();

        let i_min = *image_positions.iter().map(|((i, _), _)| i).min().unwrap();
        let j_min = *image_positions.iter().map(|((_, j), _)| j).min().unwrap();

        let image_positions = image_positions
            .into_iter()
            .map(|((i, j), tile)| {
                (
                    ((i + i_min.abs()) as usize, (j + j_min.abs()) as usize),
                    tile,
                )
            })
            .collect::<Vec<((usize, usize), Tile)>>();

        const IMAGE_TILE_HEIGHT: usize = TILE_HEIGHT - 2;
        const IMAGE_TILE_WIDTH: usize = TILE_WIDTH - 2;

        let height = *image_positions.iter().map(|((i, _), _)| i).max().unwrap() as usize + 1;
        let height = height * IMAGE_TILE_HEIGHT;
        let width = *image_positions.iter().map(|((_, j), _)| j).max().unwrap() as usize + 1;
        let width = width * IMAGE_TILE_HEIGHT;

        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut line: Vec<bool> = Vec::new();
            line.resize_with(width, Default::default);
            pixels.push(line);
        }

        for (pos, tile) in image_positions {
            let begin_i = IMAGE_TILE_HEIGHT * pos.0 as usize;
            let begin_j = IMAGE_TILE_WIDTH * pos.1 as usize;

            for i in 0..IMAGE_TILE_HEIGHT {
                for j in 0..IMAGE_TILE_WIDTH {
                    // + 1 in the tile to skip the border
                    pixels[begin_i + i][begin_j + j] = tile.get(i + 1, j + 1);
                }
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = if self.pixels[i][j] { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../input/day20_provided.txt");

    #[test]
    fn part1_provided() {
        assert_eq!(part1(PROVIDED).unwrap(), 20_899_048_083_289);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT).unwrap(), 5_775_714_912_743);
    }
}

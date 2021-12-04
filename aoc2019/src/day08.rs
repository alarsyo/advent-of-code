use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{Context, Result};

const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;

const INPUT: &str = include_str!("../input/day08.txt");

pub fn run() -> Result<String> {
    let mut res = String::with_capacity(128 + IMG_HEIGHT * IMG_WIDTH);

    let image: Image = INPUT.parse()?;

    writeln!(res, "part 1: {}", part1(&image)?)?;
    writeln!(res, "part 2:")?;
    part2(&image, &mut res)?;

    Ok(res)
}

fn part1(image: &Image) -> Result<usize> {
    let most_zero_layer = image
        .layers
        .iter()
        .min_by_key(|l| l.pixels.iter().flatten().filter(|d| **d == 0).count())
        .context("image had 0 layers...")?;

    let one_count = most_zero_layer
        .pixels
        .iter()
        .flatten()
        .filter(|d| **d == 1)
        .count();

    let two_count = most_zero_layer
        .pixels
        .iter()
        .flatten()
        .filter(|d| **d == 2)
        .count();

    Ok(one_count * two_count)
}

fn part2(image: &Image, res: &mut String) -> Result<()> {
    writeln!(res, "{}", image)?;
    Ok(())
}

struct Layer {
    pixels: Vec<Vec<u8>>,
}

struct Image {
    layers: Vec<Layer>,
    result: Vec<Vec<u8>>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.result.len() {
            for j in 0..self.result[i].len() {
                write!(f, "{}", if self.result[i][j] == 1 { '█' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_end();
        let digits: Vec<u8> = s
            .chars()
            .map(|c| c.to_digit(10))
            .filter_map(|d| d.map(|d| d as u8))
            .collect();

        let lines = digits
            .chunks(IMG_WIDTH)
            .map(<[_]>::to_vec)
            .collect::<Vec<Vec<u8>>>();

        let layers = lines
            .chunks(IMG_HEIGHT)
            .map(<[_]>::to_vec)
            .map(|pixels| Layer { pixels })
            .collect::<Vec<Layer>>();

        let mut result = vec![vec![2; IMG_WIDTH]; IMG_HEIGHT];

        // overlap layers
        for layer in &layers {
            for (src_row, dst_row) in layer.pixels.iter().zip(result.iter_mut()) {
                for (src_pixel, dst_pixel) in src_row.iter().zip(dst_row.iter_mut()) {
                    if let 2 = *dst_pixel {
                        *dst_pixel = *src_pixel;
                    }
                }
            }
        }

        Ok(Image { layers, result })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_real() {
        let image: Image = INPUT.parse().unwrap();
        assert_eq!(part1(&image).unwrap(), 1848);
    }
}

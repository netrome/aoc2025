pub fn p1(input: &str) -> String {
    let (shapes, regions) = input.split_once("\n\n\n").unwrap();
    let shapes: [Shape; 6] = shapes
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect_array()
        .unwrap();

    let regions: Vec<Region> = regions.trim().lines().map(|s| s.parse().unwrap()).collect();
    let candidates: Vec<Region> = regions
        .iter()
        .filter(|region| shape_area_is_within_region_size(&shapes, region))
        .cloned()
        .collect();

    let trivial_cases: Vec<Region> = regions
        .iter()
        .cloned()
        .filter(is_trivially_packable)
        .collect();

    println!("Shapes: {shapes:?}");
    println!("Regions: {regions:?}");
    println!("Num candidates: {}", candidates.len());
    println!("Trivial cases: {}", trivial_cases.len());

    // Lol, I spent so much time thinking about this problem
    // only to find that all the candidates could be trivially packed.
    // Oh well, this was quite fun :)

    trivial_cases.len().to_string()
}

fn shape_area_is_within_region_size(shapes: &[Shape; 6], region: &Region) -> bool {
    let shape_area: usize = region
        .quantities
        .iter()
        .zip(shapes.iter())
        .map(|(quantity, shape)| *quantity as usize * shape.indices.len())
        .sum();

    shape_area <= region.width as usize * region.height as usize
}

fn is_trivially_packable(region: &Region) -> bool {
    let trivial_pack_area = region
        .quantities
        .iter()
        .map(|quantity| *quantity as usize)
        .sum::<usize>()
        * 9;

    trivial_pack_area
        <= (region.width - region.width % 3) as usize * (region.height - region.height % 3) as usize
}

pub fn p2(_input: &str) -> String {
    // Not applicable
    todo!()
}

#[derive(Debug)]
struct Shape {
    indices: Vec<Pos>,
}

#[derive(Debug, Clone)]
struct Region {
    width: u8,
    height: u8,
    quantities: [u8; 6],
}

#[derive(Debug)]
struct Configuration {
    bl: Pos,
    tr: Pos,
    indices: HashSet<Pos>,
    quantities: [u8; 6],
}

type Pos = num::Complex<i8>;

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indices = utils::char_grid_iter::<char>(&s[2..])
            .filter_map(|(x, y, c)| (c == '#').then_some(Pos::new(x as i8, y as i8)))
            .collect();

        Ok(Self { indices })
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dimensions, quantities) = s.trim().split_once(':').unwrap();

        let quantities: [u8; 6] = quantities
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_array()
            .unwrap();

        let (height, width) = dimensions.split_once('x').unwrap();
        let height: u8 = height.parse().unwrap();
        let width: u8 = width.parse().unwrap();

        Ok(Self {
            quantities,
            width,
            height,
        })
    }
}

use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use crate::{solution::Solution, utils};
inventory::submit!(Solution::new(12, 1, p1));
inventory::submit!(Solution::new(12, 2, p2));

pub fn p1(input: &str) -> String {
    let positions: Vec<Pos> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut junction_boxes = JunctionBoxes::new(positions.clone());

    let mut pairs: Vec<_> = positions
        .iter()
        .cartesian_product(positions.iter())
        .map(|(left, right)| (left.square_distance(right), left, right))
        .filter(|(dist, _, _)| *dist > 0)
        .filter(|(_, left, right)| left < right)
        .collect();

    pairs.sort_by_key(|(dist, _, _)| *dist);

    for pair in pairs[0..1000].iter() {
        junction_boxes.join_circuits(pair.1, pair.2);
    }

    let mut lengths: Vec<_> = junction_boxes
        .circuits
        .iter()
        .map(|circuit| circuit.0.len())
        .collect();

    lengths.sort();
    lengths.reverse();

    println!("Lengths: {lengths:?}");

    lengths[0..3].iter().product::<usize>().to_string()
}

pub fn p2(input: &str) -> String {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i64, i64, i64);

#[derive(Debug)]
struct JunctionBoxes {
    circuits: Vec<Circuit>,
}

#[derive(Clone, Debug)]
struct Circuit(Vec<Pos>);

impl Pos {
    fn square_distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = sscanf::scanf!(s.trim(), "{i64},{i64},{i64}").unwrap();

        Ok(Self(x, y, z))
    }
}

impl JunctionBoxes {
    fn new(positions: impl IntoIterator<Item = Pos>) -> Self {
        Self {
            circuits: positions
                .into_iter()
                .map(|pos| Circuit(vec![pos]))
                .collect(),
        }
    }

    fn join_circuits(&mut self, pos1: &Pos, pos2: &Pos) {
        let idx1 = self
            .circuits
            .iter()
            .find_position(|circuit| circuit.0.contains(pos1))
            .unwrap()
            .0;

        let idx2 = self
            .circuits
            .iter()
            .find_position(|circuit| circuit.0.contains(pos2))
            .unwrap()
            .0;

        if idx1 != idx2 {
            let circuit_2 = self.circuits[idx2].clone();
            self.circuits[idx1].0.extend(circuit_2.0);

            self.circuits.remove(idx2);
        }
    }

    fn join_closest_circuits(&mut self) {
        let (idx1, idx2) = self.closest_non_connected_circuits();

        let circuit_2 = self.circuits[idx2].clone();
        self.circuits[idx1].0.extend(circuit_2.0);

        self.circuits.remove(idx2);
    }

    fn closest_non_connected_circuits(&self) -> (usize, usize) {
        let min = self
            .circuits
            .iter()
            .enumerate()
            .cartesian_product(self.circuits.iter().enumerate())
            .map(|((i, left), (j, right))| (left.distance(right), i, j))
            .filter(|(dist, _, _)| *dist > 0)
            .min()
            .unwrap();

        println!("Closest circuits {} {}", min.1, min.2);

        (min.1, min.2)
    }
}

impl Circuit {
    fn distance(&self, other: &Self) -> i64 {
        self.0
            .iter()
            .cartesian_product(&other.0)
            .map(|(left, right)| left.square_distance(right))
            .min()
            .unwrap()
    }
}

use std::str::FromStr;

use itertools::Itertools;

use crate::solution::Solution;
inventory::submit!(Solution::new(8, 1, p1));
inventory::submit!(Solution::new(8, 2, p2));

pub fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(find_fewest_presses)
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct Machine {
    diagram: BitVec,
    buttons: Vec<BitVec>,
}

fn find_fewest_presses(line: &str) -> usize {
    let machine: Machine = line.parse().unwrap();
    machine.fewest_presses()
}

impl Machine {
    fn fewest_presses(&self) -> usize {
        let mut distances: HashMap<BitVec, usize> = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((BitVec::new(), 0));

        while let Some((state, distance)) = queue.pop_front() {
            if state == self.diagram {
                return distance;
            }
            distances.insert(state, distance);
            for neighbor in self.neighbors(state) {
                if !distances.contains_key(&neighbor) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        panic!("Oh noooo!")
    }

    fn neighbors(&self, state: BitVec) -> Vec<BitVec> {
        self.buttons
            .iter()
            .map(|button_bits| button_bits.bitxor(state))
            .collect()
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<_> = s.trim().split_whitespace().collect();

        let diagram: BitVec = chunks[0]
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                '#' => Some(idx),
                _ => None,
            })
            .collect();

        let buttons = chunks[1..chunks.len() - 1]
            .into_iter()
            .map(|chunk| {
                chunk
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { diagram, buttons })
    }
}

type BitVec = tinybitset::TinyBitSet<u8, 2>;

use std::{
    collections::{HashMap, VecDeque},
    ops::BitXor,
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(10, 1, p1));
inventory::submit!(Solution::new(10, 2, p2));

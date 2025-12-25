pub fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(find_fewest_presses)
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(find_fewest_presses_for_joltage)
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Machine {
    diagram: BitVec,
    buttons: Vec<BitVec>,
    buttons_alt: Vec<Vec<usize>>,
    joltage_req: [u8; 12],
}

fn find_fewest_presses(line: &str) -> usize {
    let machine: Machine = line.parse().unwrap();
    machine.fewest_presses()
}

fn find_fewest_presses_for_joltage(line: &str) -> usize {
    println!("Processing line: {line}");
    let machine: Machine = line.parse().unwrap();
    dbg!(machine.fewest_presses_to_match_joltage_req())
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

    fn fewest_presses_to_match_joltage_req(&self) -> usize {
        let mut distances: HashMap<[u8; 12], usize> = HashMap::new();

        let mut heap = BinaryHeap::new();
        heap.push((0, [0; 12], 0));

        while let Some((height, state, distance)) = heap.pop() {
            if state == self.joltage_req {
                return distance;
            }
            distances.insert(state, distance);

            for (neighbor, height) in self.neighbors_joltage(state, height) {
                if !distances.contains_key(&neighbor) && !self.is_too_high(neighbor) {
                    heap.push((height, neighbor, distance + 1));
                }
            }
        }

        panic!("Nooooooooope")
    }

    fn neighbors_joltage(&self, state: [u8; 12], height: u16) -> Vec<([u8; 12], u16)> {
        self.buttons_alt
            .iter()
            .map(|buttons| {
                let mut state = state.clone();
                for idx in buttons {
                    state[*idx] += 1
                }
                (state, height + buttons.len() as u16)
            })
            .collect()
    }

    fn is_too_high(&self, state: [u8; 12]) -> bool {
        state
            .iter()
            .zip(self.joltage_req.iter())
            .any(|(state, req)| state > req)
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

        let buttons_alt = chunks[1..chunks.len() - 1]
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

        let joltage_req = chunks[chunks.len() - 1]
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(",")
            .map(|s| s.parse().unwrap())
            .enumerate()
            .fold([0; 12], |mut arr, (idx, val)| {
                arr[idx] = val;
                arr
            });

        Ok(Self {
            diagram,
            buttons,
            buttons_alt,
            joltage_req,
        })
    }
}

type BitVec = tinybitset::TinyBitSet<u8, 2>;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::BitXor,
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(10, 1, p1));
inventory::submit!(Solution::new(10, 2, p2));

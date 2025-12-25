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
    let machine: Machine = line.parse().unwrap();
    machine.fewest_presses_bifurcate()
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

    fn fewest_presses_bifurcate(&self) -> usize {
        let mut memo: HashMap<Vec<usize>, usize> = HashMap::new();
        let target: Vec<usize> = self.joltage_req[0..self.num_joltages()]
            .iter()
            .map(|&x| x as usize)
            .collect();
        self.solve_recursive(&target, &mut memo)
    }

    fn num_joltages(&self) -> usize {
        self.joltage_req
            .iter()
            .take_while(|&&x| x != 0)
            .count()
            .max(
                self.joltage_req
                    .iter()
                    .position(|&x| x != 0)
                    .map_or(0, |i| i + 1),
            )
    }

    fn solve_recursive(&self, target: &[usize], memo: &mut HashMap<Vec<usize>, usize>) -> usize {
        let target_vec = target.to_vec();

        // Base case: all joltages are 0
        if target.iter().all(|&x| x == 0) {
            return 0;
        }

        // Check memo
        if let Some(&result) = memo.get(&target_vec) {
            return result;
        }

        let num_buttons = self.buttons_alt.len();
        let mut min_presses = usize::MAX;

        // Try all possible button combinations (2^num_buttons possibilities)
        for button_mask in 0..(1 << num_buttons) {
            // Calculate the parity pattern this button combination would create
            let mut parity_state = vec![0; target.len()];
            let mut button_count = 0;

            for button_idx in 0..num_buttons {
                if (button_mask >> button_idx) & 1 == 1 {
                    button_count += 1;
                    // Add this button's effect to parity state
                    for &joltage_idx in &self.buttons_alt[button_idx] {
                        if joltage_idx < target.len() {
                            parity_state[joltage_idx] += 1;
                        }
                    }
                }
            }

            // Check if this button combination gives the right parity
            let mut valid_parity = true;
            let mut remaining_joltages = vec![0; target.len()];

            for i in 0..target.len() {
                if target[i] % 2 != parity_state[i] % 2 {
                    valid_parity = false;
                    break;
                }
                if target[i] < parity_state[i] {
                    valid_parity = false;
                    break;
                }
                remaining_joltages[i] = target[i] - parity_state[i];
            }

            if !valid_parity {
                continue;
            }

            // Check if all remaining joltages are even
            if !remaining_joltages.iter().all(|&x| x % 2 == 0) {
                continue;
            }

            // Divide by 2 and recurse
            let halved: Vec<usize> = remaining_joltages.iter().map(|&x| x / 2).collect();
            let recursive_cost = self.solve_recursive(&halved, memo);

            if recursive_cost != usize::MAX {
                let total_cost = button_count + 2 * recursive_cost;
                min_presses = min_presses.min(total_cost);
            }
        }

        let result = if min_presses == usize::MAX {
            usize::MAX
        } else {
            min_presses
        };
        memo.insert(target_vec, result);
        result
    }

    fn fewest_presses_to_match_joltage_req(&self) -> usize {
        let mut distances: HashMap<[u8; 12], usize> = HashMap::new();

        let mut stack = Vec::new();
        stack.push(([0; 12], 0));

        while let Some((state, distance)) = stack.pop() {
            if state == self.joltage_req {
                return distance;
            }
            distances.insert(state, distance);

            for neighbor in self.neighbors_joltage(state) {
                if !distances.contains_key(&neighbor) && !self.is_too_high(neighbor) {
                    stack.push((neighbor, distance + 1));
                }
            }
        }

        panic!("Nooooooooope")
    }

    fn neighbors_joltage(&self, state: [u8; 12]) -> Vec<[u8; 12]> {
        self.buttons_alt
            .iter()
            .map(|buttons| {
                let mut state = state.clone();
                for idx in buttons {
                    state[*idx] += 1
                }
                state
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

        let mut buttons_alt: Vec<Vec<usize>> = chunks[1..chunks.len() - 1]
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

        buttons_alt.sort_by_key(|buttons| usize::MAX - buttons.len());

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
    collections::{BinaryHeap, HashMap, VecDeque},
    ops::BitXor,
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(10, 1, p1));
inventory::submit!(Solution::new(10, 2, p2));

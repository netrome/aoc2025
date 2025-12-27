pub fn p1(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();

    graph
        .num_paths("you".parse().unwrap(), "out".parse().unwrap())
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut graph: Graph = input.parse().unwrap();

    let paths_from_dac_to_out = graph.num_paths("dac".parse().unwrap(), "out".parse().unwrap());
    graph.prune_v2("dac".parse().unwrap());

    let paths_from_fft_to_dac = graph.num_paths("fft".parse().unwrap(), "dac".parse().unwrap());
    graph.prune_v2("fft".parse().unwrap());

    let paths_from_svr_to_fft = graph.num_paths("svr".parse().unwrap(), "fft".parse().unwrap());

    (paths_from_svr_to_fft * paths_from_fft_to_dac * paths_from_dac_to_out).to_string()
}

pub fn p2_old(input: &str) -> String {
    let mut graph: Graph = input.parse().unwrap();

    let paths_from_dac_to_out = graph.all_paths("dac".parse().unwrap(), "out".parse().unwrap());
    println!("Dac to out");
    graph.prune(&paths_from_dac_to_out);
    println!("Pruned");
    let num_paths_from_dac_to_out = paths_from_dac_to_out.len();

    let paths_from_fft_to_dac = graph.all_paths("fft".parse().unwrap(), "dac".parse().unwrap());
    println!("FFT to to dac");
    graph.prune(&paths_from_fft_to_dac);
    println!("Pruned");
    let num_paths_from_fft_to_dac = paths_from_fft_to_dac.len();

    let paths_from_svr_to_fft = graph.all_paths("svr".parse().unwrap(), "fft".parse().unwrap());
    println!("SVR to to fft");
    graph.prune(&paths_from_svr_to_fft);
    println!("Pruned");
    let num_paths_from_svr_to_fft = paths_from_svr_to_fft.len();

    (num_paths_from_svr_to_fft * num_paths_from_fft_to_dac * num_paths_from_dac_to_out).to_string()
}

pub struct Graph(HashMap<NodeId, Vec<NodeId>>);

impl Graph {
    fn num_paths(&self, from: NodeId, to: NodeId) -> u64 {
        let mut num_paths = HashMap::new();
        let mut to_visit = VecDeque::new();

        num_paths.insert(from, 1);
        to_visit.push_back(from);

        while let Some(node) = to_visit.pop_front() {
            if node == to {
                continue;
            }

            let n = num_paths.remove(&node).unwrap();

            for neighbor in self.0.get(&node).unwrap() {
                if !num_paths.contains_key(&neighbor) {
                    to_visit.push_back(*neighbor);
                }
                *num_paths.entry(*neighbor).or_default() += n;
            }
        }

        *num_paths.get(&to).unwrap()
    }

    fn prune_v2(&mut self, from: NodeId) {
        let mut to_visit = vec![from];

        while let Some(node) = to_visit.pop() {
            let Some(neighbors) = self.0.remove(&node) else {
                continue;
            };

            for neighbor in neighbors {
                to_visit.push(neighbor);
                for list in self.0.values_mut() {
                    let Some((idx, _)) = list.iter().enumerate().find(|(_, n)| **n == neighbor)
                    else {
                        continue;
                    };

                    list.remove(idx);
                }
            }
        }
    }

    fn all_paths(&self, from: NodeId, to: NodeId) -> Vec<Vec<NodeId>> {
        let mut found_paths = Vec::new();
        let mut in_progress = vec![vec![from]];

        while let Some(path) = in_progress.pop() {
            let candidates = self.extend(path);

            for candidate in candidates {
                if *candidate.last().unwrap() == to {
                    found_paths.push(candidate);
                } else {
                    in_progress.push(candidate);
                }
            }
        }

        found_paths
    }

    fn extend(&self, path: Vec<NodeId>) -> Vec<Vec<NodeId>> {
        let mut paths = Vec::new();

        let Some(neighbors) = self.0.get(path.last().unwrap()) else {
            return vec![];
        };

        for neighbor in neighbors {
            let mut path = path.clone();
            path.push(*neighbor);
            paths.push(path);
        }

        paths
    }

    fn prune(&mut self, paths: &[Vec<NodeId>]) {
        for path in paths {
            for node_id in &path[1..] {
                self.0.remove(node_id);
            }
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct NodeId([char; 3]);

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for line in s.trim().lines() {
            let (node, neighbors) = line.trim().split_once(":").unwrap();
            let node: NodeId = node.parse().unwrap();

            for neighbor in neighbors.trim().split_whitespace() {
                let neighbor: NodeId = neighbor.parse().unwrap();

                graph.entry(node).or_default().push(neighbor);
            }
        }

        Ok(Self(graph))
    }
}

impl FromStr for NodeId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().collect::<Vec<_>>().as_slice() {
            [a, b, c] => Self([*a, *b, *c]),
            _ => anyhow::bail!("wrong size"),
        })
    }
}

use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));

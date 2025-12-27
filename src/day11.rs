pub fn p1(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();

    graph
        .all_paths("you".parse().unwrap(), "out".parse().unwrap())
        .len()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();

    graph
        .all_paths("svr".parse().unwrap(), "out".parse().unwrap())
        .len()
        .to_string()
}

pub struct Graph(HashMap<NodeId, Vec<NodeId>>);

impl Graph {
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

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));

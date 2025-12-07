pub fn p1(input: &str) -> String {
    let mut lines = input.trim().lines();
    let start = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .unwrap()
        .0;

    let all_splitters: Vec<HashSet<usize>> = lines
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(idx, _)| idx)
                .collect()
        })
        .collect();

    let mut pipes: HashSet<usize> = HashSet::from_iter([start]);
    let mut num_splits = 0;

    for splitters in all_splitters {
        let splits: Vec<usize> = pipes.intersection(&splitters).cloned().collect();
        for split in splits {
            pipes.remove(&split);
            pipes.insert(split - 1);
            pipes.insert(split + 1);
            num_splits += 1;
        }
    }

    num_splits.to_string()
}

pub fn p2(input: &str) -> String {
    let mut lines = input.trim().lines();
    let start = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .unwrap()
        .0;

    let all_splitters: Vec<HashSet<usize>> = lines
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(idx, _)| idx)
                .collect()
        })
        .collect();

    let mut timelines: HashMap<usize, usize> = HashMap::from_iter([(start, 1)]);
    let mut num_splits = 0;

    for splitters in all_splitters {
        let timelines: Vec<(usize, usize)> = timelines
            .iter()
            .filter(|(key, v)| splitters.contains(key))
            .map(|(k, v)| (*k, *v))
            .collect();
        for timeline in timelines {
            timelines.remove(timeline.0);
            pipes.insert(split - 1);
            pipes.insert(split + 1);
            num_splits += 1;
        }
    }

    num_splits.to_string()
}

use std::collections::{HashMap, HashSet};

use crate::solution::Solution;
inventory::submit!(Solution::new(7, 1, p1));
inventory::submit!(Solution::new(7, 2, p2));

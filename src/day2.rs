pub fn p1(input: &str) -> String {
    let ranges: Vec<Range> = input
        .trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect();

    ranges
        .iter()
        .flat_map(|range| range.invalid_ids())
        .sum::<i64>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    todo!()
}

#[derive(Debug, Clone)]
struct Range(i64, i64);

impl Range {
    fn invalid_ids(&self) -> Vec<i64> {
        let mut invalid_ids = Vec::new();

        let mut next_id = next_invalid_id(self.0 - 1);

        while next_id <= self.1 {
            invalid_ids.push(next_id);
            next_id = next_invalid_id(next_id);
        }

        invalid_ids
    }
}

fn next_invalid_id(mut from: i64) -> i64 {
    if from == 0 {
        from += 1
    }
    let digits = from.ilog10() + 1;

    if digits.is_odd() {
        10_i64.pow(digits / 2) + 10_i64.pow(digits)
    } else {
        next_invalid_id_even_pow(digits, from)
    }
}

fn next_invalid_id_even_pow(digits: u32, from: i64) -> i64 {
    let order = 10_i64.pow(digits / 2);
    let high = from / order;
    let high_part = high * order;
    let low = from - high_part;

    let candidate = if high > low {
        high_part + high
    } else {
        high_part + order + high + 1
    };

    if candidate.ilog10() + 1 > digits {
        next_invalid_id(candidate)
    } else {
        candidate
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = sscanf::scanf!(s, "{i64}-{i64}").unwrap();
        Ok(Range(low, high))
    }
}

use std::str::FromStr;

use num::Integer;

use crate::solution::Solution;
inventory::submit!(Solution::new(2, 1, p1));
inventory::submit!(Solution::new(2, 2, p2));

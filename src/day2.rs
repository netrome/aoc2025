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
    let ranges: Vec<Range> = input
        .trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect();

    ranges
        .iter()
        .flat_map(|range| range.invalid_ids_general())
        .sum::<i64>()
        .to_string()
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

    fn invalid_ids_general(&self) -> HashSet<i64> {
        let mut invalid_ids = HashSet::new();

        for num_parts in 2..=10 {
            let mut next_id = next_invalid_id_general(self.0 - 1, num_parts);

            while next_id <= self.1 {
                invalid_ids.insert(next_id);
                next_id = next_invalid_id_general(next_id, num_parts);
            }
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

fn next_invalid_id_general(mut from: i64, num_parts: u32) -> i64 {
    if from == 0 {
        from += 1
    }

    let digits = from.ilog10() + 1;

    if digits.rem_euclid(num_parts) == 0 {
        next_invalid_id_even_pow_general(digits, from, num_parts)
    } else {
        let next_digits = digits + num_parts - digits.rem_euclid(num_parts);
        let base = 10_i64.pow(next_digits / num_parts - 1);
        let increment = 10_i64.pow(next_digits / num_parts);
        let mut next = base;
        for _ in 1..num_parts {
            next = next * increment + base;
        }

        next
    }
}

fn next_invalid_id_even_pow_general(digits: u32, from: i64, num_parts: u32) -> i64 {
    let mut pieces = Vec::new();

    let mut remaining = from;
    for i in (0..num_parts).rev() {
        let order = 10_i64.pow((digits * i) / num_parts);
        let piece = remaining / order;
        pieces.push(piece);
        remaining -= piece * order;
    }

    let first_piece = pieces[0];

    let candidate = if pieces[1..].iter().all(|piece| *piece <= first_piece)
        && !pieces[1..].iter().all(|piece| *piece == first_piece)
    {
        let mut candidate = 0;

        for i in (0..num_parts).rev() {
            let order = 10_i64.pow((digits * i) / num_parts);
            candidate += first_piece * order;
        }

        candidate
    } else {
        let piece = first_piece + 1;

        let mut candidate = 0;

        for i in (0..num_parts).rev() {
            let order = 10_i64.pow((digits * i) / num_parts);
            candidate += piece * order;
        }

        candidate
    };

    if candidate.ilog10() + 1 > digits {
        next_invalid_id_general(candidate, num_parts)
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

use std::{collections::HashSet, str::FromStr};

use num::Integer;

use crate::solution::Solution;
inventory::submit!(Solution::new(2, 1, p1));
inventory::submit!(Solution::new(2, 2, p2));

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    #[test]
    fn next_invalid_id_general__should_give_expected_result() {
        let cases = [
            (0, 2, 11),
            (0, 3, 111),
            (98, 2, 99),
            (99, 3, 111),
            (0, 9, 111111111),
            (1001001, 9, 111111111),
            (52, 2, 55),
            (101, 2, 1010),
        ];

        for (from, num_parts, expected) in cases {
            let result = next_invalid_id_general(from, num_parts);
            assert_eq!(result, expected, "{from} {num_parts}");
        }
    }
}

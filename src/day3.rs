pub fn p1(input: &str) -> String {
    let banks: Vec<Vec<u64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    banks
        .iter()
        .map(|bank| largest_possible_joltage(&bank))
        .sum::<u64>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let banks: Vec<Vec<u64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    banks
        .iter()
        .map(|bank| largest_possible_joltage_p2(&bank, 12))
        .sum::<u64>()
        .to_string()
}

fn largest_possible_joltage(bank: &[u64]) -> u64 {
    let (largest_idx, largest) = find_largest(&bank[0..bank.len() - 1]);
    let (_, second_largest_in_remainder) = find_largest(&bank[largest_idx + 1..]);

    largest * 10 + second_largest_in_remainder
}

fn largest_possible_joltage_p2(bank: &[u64], n: usize) -> u64 {
    let mut start_idx = 0;
    let mut joltage = 0;

    for i in 1..n + 1 {
        let (largest_idx, largest) = find_largest(&bank[start_idx..bank.len() - n + i]);
        start_idx += largest_idx + 1;
        joltage = joltage * 10 + largest;
    }

    joltage
}

fn find_largest(bank: &[u64]) -> (usize, u64) {
    let max = bank.iter().max().unwrap();
    let max_idx = bank.iter().find_position(|val| *val == max).unwrap().0;

    (max_idx, *max)
}

use itertools::Itertools;

use crate::solution::Solution;
inventory::submit!(Solution::new(3, 1, p1));
inventory::submit!(Solution::new(3, 2, p2));

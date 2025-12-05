pub fn p1(input: &str) -> String {
    let db = Database::parse(input);
    db.available_fresh_ingredients().len().to_string()
}

pub fn p2(input: &str) -> String {
    todo!()
}

struct Database {
    ranges: Vec<Range>,
    available_ingredients: Vec<i64>,
}

impl Database {
    fn parse(input: &str) -> Self {
        let (ranges_part, available_ingredients_part) = input.split_once("\n\n").unwrap();

        let ranges = ranges_part
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        let available_ingredients = available_ingredients_part
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Self {
            ranges,
            available_ingredients,
        }
    }

    fn available_fresh_ingredients(&self) -> Vec<i64> {
        self.available_ingredients
            .iter()
            .cloned()
            .filter(|ingredient| self.ranges.iter().any(|range| range.contains(*ingredient)))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Range(i64, i64);

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = sscanf::scanf!(s, "{i64}-{i64}").unwrap();
        Ok(Range(low, high))
    }
}

impl Range {
    fn contains(&self, val: i64) -> bool {
        val >= self.0 && val <= self.1
    }
}

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(5, 1, p1));
inventory::submit!(Solution::new(5, 2, p2));

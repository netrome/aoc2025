pub fn p1(input: &str) -> String {
    let mut db = Database::parse(input);
    db.simplify_ranges();
    db.available_fresh_ingredients().len().to_string()
}

pub fn p2(input: &str) -> String {
    let mut db = Database::parse(input);
    db.simplify_ranges();
    db.total_number_of_fresh_ingredients().to_string()
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

    fn total_number_of_fresh_ingredients(&self) -> i64 {
        self.ranges.iter().map(|range| range.size()).sum()
    }

    fn simplify_ranges(&mut self) {
        self.ranges.sort_by_key(|range| range.0);

        let mut idx = 0;
        while let (Some(left), Some(right)) = (self.ranges.get(idx), self.ranges.get(idx + 1)) {
            if let Some(merged) = left.try_merge(right) {
                self.ranges.remove(idx);
                self.ranges.remove(idx);
                self.ranges.insert(idx, merged);
            } else {
                idx += 1;
            }
        }
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

    fn size(&self) -> i64 {
        self.1 - self.0 + 1
    }

    fn try_merge(&self, other: &Self) -> Option<Self> {
        if self.0 <= other.1 && self.1 >= other.0 {
            Some(Self(self.0, other.1.max(self.1)))
        } else {
            None
        }
    }
}

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(5, 1, p1));
inventory::submit!(Solution::new(5, 2, p2));

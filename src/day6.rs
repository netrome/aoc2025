pub fn p1(input: &str) -> String {
    let lines: Vec<_> = input.trim().lines().collect();
    let (last, rest) = lines.split_last().unwrap();

    let numbers: Vec<Vec<i64>> = rest
        .into_iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<Op> = last
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let problems = MathProblems::extract(&numbers, &ops);

    problems.solve_all_sum().to_string()
}

pub fn p2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let mut ops: Vec<Op> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    ops.reverse();

    let chars: Vec<Vec<char>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>()[0..line.len()].to_vec())
        .collect();

    let mut numbers = Vec::new();
    let mut number_row: Vec<i64> = Vec::new();
    for col in (0..chars[0].len()).rev() {
        let mut line: Vec<char> = Vec::new();

        for row in 0..chars.len() {
            line.push(chars[row][col]);
        }

        let line: String = line.iter().filter(|c| !c.is_whitespace()).collect();
        let Ok(number) = line.parse() else {
            numbers.push(number_row);
            number_row = Vec::new();
            continue;
        };

        number_row.push(number);
    }
    numbers.push(number_row);

    let problems = MathProblems(
        numbers
            .into_iter()
            .zip(ops)
            .map(|(numbers, op)| MathProblem { op, numbers })
            .collect(),
    );

    problems.solve_all_sum().to_string()
}

struct MathProblems(Vec<MathProblem>);

struct MathProblem {
    op: Op,
    numbers: Vec<i64>,
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Mul,
    Add,
}

impl MathProblems {
    fn extract(numbers: &[Vec<i64>], ops: &[Op]) -> Self {
        let mut problems: Vec<MathProblem> = ops
            .iter()
            .copied()
            .map(|op| MathProblem {
                op,
                numbers: Vec::new(),
            })
            .collect();

        for row in numbers {
            for (idx, num) in row.iter().enumerate() {
                problems[idx].numbers.push(*num);
            }
        }

        Self(problems)
    }

    fn solve_all_sum(&self) -> i64 {
        self.0.iter().map(|prob| prob.solve()).sum()
    }
}

impl MathProblem {
    fn solve(&self) -> i64 {
        match self.op {
            Op::Mul => self.solve_mul(),
            Op::Add => self.solve_add(),
        }
    }

    fn solve_mul(&self) -> i64 {
        self.numbers.iter().product()
    }

    fn solve_add(&self) -> i64 {
        self.numbers.iter().sum()
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("Noooo"),
        })
    }
}

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(6, 1, p1));
inventory::submit!(Solution::new(6, 2, p2));

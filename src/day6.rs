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
    todo!()
}

struct MathProblems(Vec<MathProblem>);

struct MathProblem {
    op: Op,
    numbers: Vec<i64>,
}

#[derive(Copy, Clone)]
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

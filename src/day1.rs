pub fn p1(input: &str) -> String {
    let (_dial, num_zeros) = input
        .trim()
        .lines()
        .fold((50, 0), |(dial, mut num_zeros), line| {
            let val: i32 = line[1..].trim().parse().unwrap();
            let dial: i32 = match line.chars().next().unwrap() {
                'L' => dial - val,
                'R' => dial + val,
                _ => panic!("nope"),
            }
            .rem_euclid(100);

            if dial == 0 {
                num_zeros += 1;
            }

            (dial, num_zeros)
        });

    num_zeros.to_string()
}

pub fn p2(input: &str) -> String {
    let (_dial, num_zeros) = input
        .trim()
        .lines()
        .fold((50, 0), |(dial, mut num_zeros), line| {
            let val: i32 = line[1..].trim().parse().unwrap();
            let (dial, zeros): (i32, i32) = match line.chars().next().unwrap() {
                'L' => (
                    dial - val,
                    (val - dial).div_euclid(100) + 1 - if dial == 0 { 1 } else { 0 },
                ),
                'R' => (dial + val, (dial + val).div_euclid(100)),
                _ => panic!("nope"),
            };

            println!("{line}: Dial {dial} npz {zeros}");

            num_zeros += zeros;

            let dial = dial.rem_euclid(100);

            (dial, num_zeros)
        });

    num_zeros.to_string()
}

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));

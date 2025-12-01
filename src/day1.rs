pub fn p1(input: &str) -> String {
    let (_dial, num_zeros) = input.trim().lines().map(|line| line.split_at(1)).fold(
        (50, 0),
        |(dial, num_zeros), (dir, val)| {
            let val = val.parse::<i32>().unwrap();
            let dial: i32 = match dir {
                "L" => dial - val,
                "R" => dial + val,
                _ => panic!("nope"),
            }
            .rem_euclid(100);

            (dial, num_zeros + ((dial == 0) as i32))
        },
    );

    num_zeros.to_string()
}

pub fn p2(input: &str) -> String {
    let (_dial, num_zeros) = input.trim().lines().map(|line| line.split_at(1)).fold(
        (50, 0),
        |(dial, num_zeros), (dir, val)| {
            let val = val.parse::<i32>().unwrap();
            let (dial, zeros): (i32, i32) = match dir {
                "L" => (
                    dial - val,
                    (val - dial).div_euclid(100) + ((dial != 0) as i32),
                ),
                "R" => (dial + val, (dial + val).div_euclid(100)),
                _ => panic!("nope"),
            };

            (dial.rem_euclid(100), num_zeros + zeros)
        },
    );

    num_zeros.to_string()
}

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));

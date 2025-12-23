pub fn p1(input: &str) -> String {
    let positions: Vec<_> = input.trim().lines().map(parse_line).collect();

    let rects: HashSet<Rect> = positions
        .iter()
        .cartesian_product(positions.iter())
        .map(|(c1, c2)| Rect::from_corners(*c1, *c2))
        .collect();

    let mut rects = Vec::from_iter(rects);

    rects.sort_by_key(|rect| -rect.area());

    rects[0].area().to_string()
}

pub fn p2(input: &str) -> String {
    let pivot_1 = Pos::new(94646, 50020);
    let pivot_2 = Pos::new(94646, 48765);

    let positions: Vec<_> = input.trim().lines().map(parse_line).collect();
    let rects: HashSet<Rect> = positions
        .iter()
        .map(|pos| {
            let pivot = if pos.im > pivot_1.im {
                pivot_1
            } else {
                pivot_2
            };
            Rect::from_corners(*pos, pivot)
        })
        .collect();

    let mut rects = Vec::from_iter(rects);

    rects.sort_by_key(|rect| -rect.area());
    rects
        .into_iter()
        .filter(|rect| !positions.iter().any(|pos| rect.contains(*pos)))
        .next()
        .unwrap()
        .area()
        .to_string()
}

fn parse_line(line: &str) -> Pos {
    let (x, y) = line.trim().split_once(",").unwrap();
    Pos::new(x.parse().unwrap(), y.parse().unwrap())
}

type Pos = num::Complex<i64>;

#[derive(Hash, PartialEq, Eq)]
struct Rect {
    bl: Pos,
    tr: Pos,
}

impl Rect {
    fn from_corners(c1: Pos, c2: Pos) -> Self {
        let bl = Pos::new(c1.re.min(c2.re), c1.im.min(c2.im));
        let tr = Pos::new(c1.re.max(c2.re), c1.im.max(c2.im));

        Self { bl, tr }
    }

    fn area(&self) -> i64 {
        (self.tr.re + 1 - self.bl.re) * (self.tr.im + 1 - self.bl.im)
    }

    fn contains(&self, pos: Pos) -> bool {
        self.bl.im < pos.im && pos.im < self.tr.im && self.bl.re < pos.re && pos.re < self.tr.re
    }
}

use std::collections::HashSet;

use itertools::Itertools;

use crate::{solution::Solution, utils};
inventory::submit!(Solution::new(9, 1, p1));
inventory::submit!(Solution::new(9, 2, p2));

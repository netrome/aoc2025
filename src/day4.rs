pub fn p1(input: &str) -> String {
    let grid = Grid(
        utils::char_grid_iter(input)
            .map(|(x, y, char)| (Pos::new(x as i64, y as i64), char))
            .collect(),
    );
    grid.0
        .keys()
        .filter(|pos| {
            grid.0.get(pos).copied().unwrap_or('.') == '@' && grid.forklift_can_access(pos)
        })
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut grid = Grid(
        utils::char_grid_iter(input)
            .map(|(x, y, char)| (Pos::new(x as i64, y as i64), char))
            .collect(),
    );

    let mut total_removed = 0;
    let mut just_removed = grid.remove_reachable_paper();

    while just_removed > 0 {
        total_removed += just_removed;
        just_removed = grid.remove_reachable_paper();
    }

    total_removed.to_string()
}

type Pos = num::Complex<i64>;

#[derive(Debug)]
struct Grid(HashMap<Pos, char>);

impl Grid {
    fn forklift_can_access(&self, pos: &Pos) -> bool {
        neighbours(pos)
            .iter()
            .filter(|n| self.0.get(n).copied().unwrap_or('.') == '@')
            .count()
            < 4
    }

    fn remove_reachable_paper(&mut self) -> usize {
        let removable_positions: Vec<_> = self
            .0
            .keys()
            .filter(|pos| {
                self.0.get(pos).copied().unwrap_or('.') == '@' && self.forklift_can_access(pos)
            })
            .cloned()
            .collect();

        for pos in &removable_positions {
            self.0.insert(*pos, '.');
        }

        removable_positions.len()
    }
}

fn neighbours(pos: &Pos) -> [Pos; 8] {
    [
        pos + Pos::new(1, -1),
        pos + Pos::new(1, 0),
        pos + Pos::new(1, 1),
        pos + Pos::new(0, -1),
        pos + Pos::new(0, 1),
        pos + Pos::new(-1, -1),
        pos + Pos::new(-1, 0),
        pos + Pos::new(-1, 1),
    ]
}

use std::collections::HashMap;

use crate::{solution::Solution, utils};
inventory::submit!(Solution::new(4, 1, p1));
inventory::submit!(Solution::new(4, 2, p2));

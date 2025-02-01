use std::collections::{HashMap, VecDeque};

use aoc_utils::{Coord, Direction, out_of_bounds};

fn is_open(coord: &Coord, fav: isize) -> bool {
    let x = coord.0;
    let y = coord.1;
    let result = (x * x + 3 * x + 2 * x * y + y + y * y) + fav;
    let count = result.count_ones();
    count % 2 == 0
}

fn main() {
    let fav = 1350;
    let destination = (31, 39);

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let mut seen = HashMap::new();
    let mut positions = VecDeque::new();
    positions.push_back(((1, 1), 0));

    while let Some((coord, steps)) = positions.pop_front() {
        seen.insert(coord, steps);
        for d in directions {
            let new_coord = d.next_coord(&coord);
            let current_steps = seen.get(&new_coord);
            let better = current_steps.is_none() || *current_steps.unwrap() > steps;

            if !better || out_of_bounds(&new_coord, isize::MAX) || !is_open(&new_coord, fav) {
                continue;
            } else {
                positions.push_back((new_coord, steps + 1));
            }
        }
    }
    println!("{}", seen.get(&destination).unwrap());
    println!("{}", seen.iter().filter(|(_, s)| **s <= 50).count());
}

use std::collections::HashSet;

use aoc_utils::{Cli, get_entire_puzzle};

#[derive(Debug, Clone, Copy)]
enum Direction {
    L(i32),
    R(i32),
}

impl Direction {
    fn parse(input: &str) -> Self {
        if input.contains("L") {
            Self::L(input.strip_prefix("L").unwrap().parse().unwrap())
        } else {
            Self::R(input.strip_prefix("R").unwrap().parse().unwrap())
        }
    }

    fn distance(&self) -> i32 {
        match self {
            Direction::L(d) => *d,
            Direction::R(d) => *d,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    N,
    E,
    S,
    W,
}

impl Orientation {
    fn rotate(self, direction: Direction) -> Self {
        match self {
            Orientation::N => match direction {
                Direction::L(_) => Self::W,
                Direction::R(_) => Self::E,
            },
            Orientation::E => match direction {
                Direction::L(_) => Self::N,
                Direction::R(_) => Self::S,
            },
            Orientation::S => match direction {
                Direction::L(_) => Self::E,
                Direction::R(_) => Self::W,
            },
            Orientation::W => match direction {
                Direction::L(_) => Self::S,
                Direction::R(_) => Self::N,
            },
        }
    }

    fn advance(&self, position: &mut (i32, i32), distance: i32) {
        match self {
            Orientation::N => position.1 += distance,
            Orientation::E => position.0 += distance,
            Orientation::S => position.1 -= distance,
            Orientation::W => position.0 -= distance,
        };
    }
}

fn main() {
    let puzzle = get_entire_puzzle("input");
    let puzzle = puzzle.first().unwrap();

    let directions: Vec<_> = puzzle.split(", ").map(Direction::parse).collect();

    let result = if Cli::parse_args().part_two {
        part_two(&directions)
    } else {
        part_one(&directions)
    };

    println!("{}", result);
}

fn part_one(directions: &[Direction]) -> i32 {
    let mut start = Orientation::N;

    let mut position = (0, 0);
    for d in directions {
        start = start.rotate(*d);
        start.advance(&mut position, d.distance());
    }
    position.0.abs() + position.1.abs()
}

fn part_two(directions: &[Direction]) -> i32 {
    let mut start = Orientation::N;

    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);
    for d in directions {
        start = start.rotate(*d);

        for _ in 0..d.distance() {
            start.advance(&mut position, 1);
            if !visited.insert(position) {
                return position.0.abs() + position.1.abs();
            }
        }
    }
    i32::MAX
}

use std::collections::HashSet;

use aoc_utils::{get_entire_puzzle, Cli, Direction};

fn main() {
    let input = get_entire_puzzle("input");
    let input = input.first().unwrap();
    let houses = if Cli::parse_args().part_two {
        part_two(input)
    } else {
        part_one(input)
    };
    println!("{houses}")
}

fn part_one(input: &str) -> usize {
    let mut coords = HashSet::new();
    let mut position = (0, 0);
    coords.insert(position);

    for c in input.chars() {
        let direction = match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        };
        position = direction.next_coord(&position);
        coords.insert(position);
    }
    coords.len()
}

fn part_two(input: &str) -> usize {
    let mut coords = HashSet::new();
    let mut position = (0, 0);
    let mut robo_position = (0, 0);
    coords.insert(position);

    for (i, c) in input.chars().enumerate() {
        let direction = match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        };
        if i % 2 == 0 {
            position = direction.next_coord(&position);
            coords.insert(position);
        } else {
            robo_position = direction.next_coord(&robo_position);
            coords.insert(robo_position);
        }
    }
    coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let result = part_one(">");
        assert_eq!(result, 2);
    }

    #[test]
    fn part_one_example2() {
        let result = part_one("^>v<");
        assert_eq!(result, 4);
    }

    #[test]
    fn part_one_example3() {
        let result = part_one("^v^v^v^v^v");
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two_example1() {
        let result = part_two("^v");
        assert_eq!(result, 3);
    }

    #[test]
    fn part_two_example2() {
        let result = part_two("^>v<");
        assert_eq!(result, 3);
    }

    #[test]
    fn part_two_example3() {
        let result = part_two("^v^v^v^v^v");
        assert_eq!(result, 11);
    }
}

use aoc_utils::{Cli, get_entire_puzzle};
use itertools::Itertools;

fn is_trap(prev: &[bool; 3]) -> bool {
    if prev[0] && prev[1] && !prev[2] {
        return true;
    }
    if !prev[0] && prev[1] && prev[2] {
        return true;
    }
    if prev[0] && !prev[1] && !prev[2] {
        return true;
    }
    if !prev[0] && !prev[1] && prev[2] {
        return true;
    }
    false
}

fn main() {
    let puzzle = get_entire_puzzle("input");

    let first_line: Vec<_> = puzzle.first().unwrap().chars().map(|c| c == '^').collect();
    let mut all_lines = vec![first_line];
    let rows = if Cli::parse_args().part_two {
        400000
    } else {
        40
    };
    for _ in 0..(rows - 1) {
        let mut last_line = all_lines.last().unwrap().clone();
        last_line.push(false);
        last_line.insert(0, false);

        let mut new_line = vec![];
        for (a, b, c) in last_line.iter().tuple_windows() {
            new_line.push(is_trap(&[*a, *b, *c]));
        }

        all_lines.push(new_line);
    }

    let safe: usize = all_lines
        .iter()
        .map(|line| line.iter().filter(|s| !**s).count())
        .sum();

    println!("{safe}");
}

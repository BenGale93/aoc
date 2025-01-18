use std::{cmp::Ordering, collections::HashMap};

use aoc_utils::puzzle_input_lines;

fn transpose(message: &[Vec<char>]) -> Vec<Vec<char>> {
    let columns = message[0].len();
    let mut new_message = vec![vec![]; columns];

    for row in message {
        for (i, column) in row.iter().enumerate() {
            new_message[i].push(*column);
        }
    }
    new_message
}

fn count_and_sort(line: &[char]) -> Vec<(char, i32)> {
    let mut counts = HashMap::new();

    for c in line {
        *counts.entry(*c).or_insert(0) += 1;
    }
    let mut ordered_counts: Vec<_> = counts.into_iter().collect();
    ordered_counts.sort_by(|a, b| {
        let cmp = b.1.cmp(&a.1);
        if cmp != Ordering::Equal {
            return cmp;
        }
        a.0.cmp(&b.0)
    });
    ordered_counts
}

fn main() {
    let puzzle = puzzle_input_lines("input");

    let message: Vec<Vec<_>> = puzzle
        .map(Result::unwrap)
        .map(|l| l.chars().collect())
        .collect();

    let new_message = transpose(&message);

    let character_counts: Vec<_> = new_message.iter().map(|l| count_and_sort(l)).collect();

    let part_one: String = character_counts
        .iter()
        .map(|v| v.first().unwrap().0)
        .collect();
    let part_two: String = character_counts
        .iter()
        .map(|v| v.last().unwrap().0)
        .collect();

    println!("{part_one}");
    println!("{part_two}");
}

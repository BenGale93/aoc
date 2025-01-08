use aoc_utils::{Cli, puzzle_input_lines};
use itertools::Itertools;

fn main() {
    let weights: Vec<usize> = puzzle_input_lines("input")
        .map(|n| n.unwrap().parse().unwrap())
        .collect();

    let parts: usize = if Cli::parse_args().part_two { 4 } else { 3 };

    let result = compute_qe(&weights, parts);

    println!("{}", result);
}

fn compute_qe(weights: &[usize], parts: usize) -> usize {
    let total_weight: usize = weights.iter().sum();
    let group_weight = total_weight / parts;
    for i in 1..=weights.len() {
        let qe: Vec<_> = weights
            .iter()
            .combinations(i)
            .filter(|c| c.iter().copied().sum::<usize>() == group_weight)
            .map(|v| v.into_iter().product::<usize>())
            .collect();
        if !qe.is_empty() {
            return *qe.iter().min().unwrap();
        }
    }
    usize::MAX
}

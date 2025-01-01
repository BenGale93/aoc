use aoc_utils::{Cli, puzzle_input_lines};
use itertools::Itertools;

fn main() {
    let containers: Vec<u64> = puzzle_input_lines("input")
        .map(|n| n.unwrap().parse().unwrap())
        .collect();

    let total_eggnog = 150;

    let answer = if Cli::parse_args().part_two {
        compute_minimum(total_eggnog, &containers)
    } else {
        compute_combinations(total_eggnog, &containers)
    };

    println!("{answer}");
}

fn compute_combinations(total_eggnog: u64, containers: &[u64]) -> u64 {
    let mut combinations = 0;
    let total_containers = containers.len();
    for i in 1..=total_containers {
        for c in containers.iter().combinations(i) {
            let total: u64 = c.into_iter().sum();
            if total == total_eggnog {
                combinations += 1;
            }
        }
    }

    combinations
}

fn compute_minimum(total_eggnog: u64, containers: &[u64]) -> u64 {
    let mut combinations = 0;
    let total_containers = containers.len();
    for i in 1..=total_containers {
        for c in containers.iter().combinations(i) {
            let total: u64 = c.into_iter().sum();
            if total == total_eggnog {
                combinations += 1;
            }
        }
        if combinations > 0 {
            break;
        }
    }
    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example_one() {
        let total_eggnog = 25;
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(compute_combinations(total_eggnog, &containers), 4);
    }

    #[test]
    fn part_two_example_one() {
        let total_eggnog = 25;
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(compute_minimum(total_eggnog, &containers), 3);
    }
}

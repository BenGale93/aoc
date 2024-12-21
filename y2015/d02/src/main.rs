use aoc_utils::{puzzle_input_lines, Cli};
use itertools::Itertools;

fn main() {
    let func = if Cli::parse_args().part_two {
        part_two
    } else {
        part_one
    };
    let lines = puzzle_input_lines("input");
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split('x').collect();
        let mut nums: Vec<u32> = nums.iter().map(|n| n.parse().unwrap()).collect();
        nums.sort();
        total += func(&nums);
    }

    println!("{total}");
}

fn part_one(nums: &[u32]) -> u32 {
    let area = 2 * nums
        .iter()
        .combinations(2)
        .map(|c| c.into_iter().product::<u32>())
        .sum::<u32>();

    let small: u32 = nums.iter().take(2).product();
    area + small
}

fn part_two(nums: &[u32]) -> u32 {
    let wrap = nums.iter().product::<u32>();

    let small: u32 = 2 * nums.iter().take(2).sum::<u32>();

    wrap + small
}

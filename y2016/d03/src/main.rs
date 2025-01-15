use aoc_utils::{Cli, puzzle_input_lines};
use itertools::Itertools;

fn main() {
    let mut triangles: Vec<_> = puzzle_input_lines("input")
        .map(Result::unwrap)
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let result = if Cli::parse_args().part_two {
        part_two(&mut triangles)
    } else {
        part_one(&mut triangles)
    };

    println!("{result}")
}

fn part_one(triangles: &mut [Vec<u64>]) -> usize {
    triangles
        .iter_mut()
        .map(|t| {
            t.sort();
            t
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}

fn part_two(triangles: &mut [Vec<u64>]) -> usize {
    let mut valid = 0;
    for c in &triangles.iter().chunks(3) {
        let chunk: Vec<_> = c.collect();
        let mut sub_triangles = vec![];
        for i in 0..3 {
            sub_triangles.push(vec![chunk[0][i], chunk[1][i], chunk[2][i]]);
        }
        valid += part_one(&mut sub_triangles);
    }
    valid
}

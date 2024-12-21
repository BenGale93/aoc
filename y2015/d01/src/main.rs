use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let cli = Cli::parse_args();

    let input = get_entire_puzzle("input");
    let input = input.first().unwrap();

    let output = if cli.part_two {
        part_two(input)
    } else {
        part_one(input)
    };

    print!("{output}");
}

fn part_one(input: &str) -> isize {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor += -1,
            _ => continue,
        }
    }
    floor
}

fn part_two(input: &str) -> isize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor += -1,
            _ => continue,
        }
        if floor == -1 {
            return (i + 1) as isize;
        }
    }
    panic!()
}

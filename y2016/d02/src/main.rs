use aoc_utils::{Cli, Coord, Direction, get_entire_puzzle};

fn parse_direction(d: char) -> Direction {
    match d {
        'U' => Direction::Up,
        'L' => Direction::Left,
        'R' => Direction::Right,
        'D' => Direction::Down,
        _ => panic!(),
    }
}

fn coord_map_part_one(coord: Coord) -> &'static str {
    match coord {
        (1, 1) => "1",
        (1, 2) => "2",
        (1, 3) => "3",
        (2, 1) => "4",
        (2, 2) => "5",
        (2, 3) => "6",
        (3, 1) => "7",
        (3, 2) => "8",
        (3, 3) => "9",
        _ => panic!(),
    }
}

fn compute_digit_part_one(directions: &[Vec<Direction>]) -> String {
    let mut coord = (2, 2);
    let mut output = vec![];

    for line in directions {
        for d in line {
            coord = d.next_coord(&coord);
            coord.0 = coord.0.clamp(1, 3);
            coord.1 = coord.1.clamp(1, 3);
        }
        output.push(coord_map_part_one(coord));
    }

    output.join("")
}

fn coord_map_part_two(coord: Coord) -> &'static str {
    match coord {
        (-2, 0) => "1",
        (-1, -1) => "2",
        (-1, 0) => "3",
        (-1, 1) => "4",
        (0, -2) => "5",
        (0, -1) => "6",
        (0, 0) => "7",
        (0, 1) => "8",
        (0, 2) => "9",
        (1, -1) => "A",
        (1, 0) => "B",
        (1, 1) => "C",
        (2, 0) => "D",
        _ => panic!(),
    }
}

fn compute_digit_part_two(directions: &[Vec<Direction>]) -> String {
    let mut coord = (0, -2);
    let mut output = vec![];

    for line in directions {
        for d in line {
            let next_coord = d.next_coord(&coord);

            if (next_coord.0.abs() + next_coord.1.abs()) <= 2 {
                coord = next_coord;
            }
        }
        output.push(coord_map_part_two(coord));
    }
    output.join("")
}

fn main() {
    let puzzle = get_entire_puzzle("input");

    let directions: Vec<Vec<_>> = puzzle
        .iter()
        .map(|l| l.chars().map(parse_direction).collect())
        .collect();

    let compute = if Cli::parse_args().part_two {
        compute_digit_part_two
    } else {
        compute_digit_part_one
    };

    let digits = compute(&directions);

    dbg!(&digits);
}

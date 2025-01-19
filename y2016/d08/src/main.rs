use std::sync::OnceLock;

use aoc_utils::puzzle_input_lines;
use regex::Regex;

fn rect_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"rect (\d*)x(\d*)").unwrap())
}

fn row_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"rotate row y=(\d*) by (\d*)").unwrap())
}

fn column_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"rotate column x=(\d*) by (\d*)").unwrap())
}

#[derive(Debug)]
enum Instruction {
    Rect((usize, usize)),
    Row((usize, usize)),
    Column((usize, usize)),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        if let Some(capture) = rect_regex().captures(input) {
            return Self::Rect((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }

        if let Some(capture) = row_regex().captures(input) {
            return Self::Row((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }

        if let Some(capture) = column_regex().captures(input) {
            return Self::Column((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }
        panic!()
    }

    fn run(&self, screen: &mut Screen) {
        match self {
            Instruction::Rect(r) => turn_on_rect(screen, r),
            Instruction::Row(r) => rotate_row(screen, r),
            Instruction::Column(c) => rotate_column(screen, c),
        }
    }
}

fn turn_on_rect(screen: &mut Screen, rect: &(usize, usize)) {
    for row in 0..rect.0 {
        for col in 0..rect.1 {
            screen[row][col] = true;
        }
    }
}

fn rotate_column(screen: &mut Screen, column: &(usize, usize)) {
    let current_col = screen[column.0];
    let col_len = current_col.len();

    for i in 0..col_len {
        let offset = (i as isize - column.1 as isize).rem_euclid(col_len as isize);
        screen[column.0][i] = current_col[offset as usize];
    }
}

fn rotate_row(screen: &mut Screen, row: &(usize, usize)) {
    let row_len = screen.len();
    let current_row: Vec<_> = screen.iter().map(|c| c[row.0]).collect();

    for i in 0..row_len {
        let offset = (i as isize - row.1 as isize).rem_euclid(row_len as isize);
        screen[i][row.0] = current_row[offset as usize];
    }
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

type Screen = [[bool; HEIGHT]; WIDTH];

fn main() {
    let puzzle = puzzle_input_lines("input");

    let instructions: Vec<Instruction> = puzzle
        .map(Result::unwrap)
        .map(|l| Instruction::parse(&l))
        .collect();

    let mut screen: Screen = [[false; HEIGHT]; WIDTH];

    for i in instructions {
        i.run(&mut screen);
    }

    for col in screen {
        for row in col.iter().rev() {
            if *row { print!("#") } else { print!(".") }
        }
        println!();
    }

    let total: usize = screen
        .into_iter()
        .map(|r| r.into_iter().filter(|p| *p).count())
        .sum();

    println!("{total}");
}

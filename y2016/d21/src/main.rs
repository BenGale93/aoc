#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use std::{ascii, collections::VecDeque, sync::OnceLock};

use aoc_utils::{Cli, puzzle_input_lines};
use regex::Regex;

fn swap_pos_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"swap position (\d*) with position (\d*)").unwrap())
}

fn swap_letter_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap())
}

fn rotate_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"rotate (left|right) (\d*) step").unwrap())
}

fn rotate_based_on_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"rotate based on position of letter ([a-z])").unwrap())
}

fn reverse_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"reverse positions (\d*) through (\d*)").unwrap())
}

fn move_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"move position (\d*) to position (\d*)").unwrap())
}

#[derive(Debug)]
enum Instruction {
    SwapPos((usize, usize)),
    SwapLetter((ascii::Char, ascii::Char)),
    Rotate(isize),
    RotateBasedOn(ascii::Char),
    Reverse((usize, usize)),
    Move((usize, usize)),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        if let Some(capture) = swap_pos_regex().captures(input) {
            return Self::SwapPos((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }

        if let Some(capture) = swap_letter_regex().captures(input) {
            return Self::SwapLetter((
                *capture[1].as_ascii().unwrap().first().unwrap(),
                *capture[2].as_ascii().unwrap().first().unwrap(),
            ));
        }

        if let Some(capture) = rotate_regex().captures(input) {
            let direction = &capture[1];
            let mut magnitude: isize = capture[2].parse().unwrap();

            if direction == "left" {
                magnitude = -magnitude;
            }

            return Self::Rotate(magnitude);
        }

        if let Some(capture) = rotate_based_on_regex().captures(input) {
            return Self::RotateBasedOn(*capture[1].as_ascii().unwrap().first().unwrap());
        }

        if let Some(capture) = reverse_regex().captures(input) {
            return Self::Reverse((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }

        if let Some(capture) = move_regex().captures(input) {
            return Self::Move((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }
        panic!("{}", input)
    }

    fn run(&self, phrase: &mut [ascii::Char]) {
        match self {
            Instruction::SwapPos((x, y)) => position_swap(phrase, *x, *y),
            Instruction::SwapLetter((letter_x, letter_y)) => {
                letter_swap(phrase, *letter_x, *letter_y)
            }
            Instruction::Rotate(distance) => rotate(phrase, *distance),
            Instruction::RotateBasedOn(ascii_char) => rotate_based_on(phrase, *ascii_char),
            Instruction::Reverse((x, y)) => reverse_positions(phrase, *x, *y),
            Instruction::Move((x, y)) => move_position(phrase, *x, *y),
        }
    }

    fn run_reverse(&self, phrase: &mut [ascii::Char]) {
        match self {
            Instruction::SwapPos((x, y)) => position_swap(phrase, *x, *y),
            Instruction::SwapLetter((letter_x, letter_y)) => {
                letter_swap(phrase, *letter_x, *letter_y)
            }
            Instruction::Rotate(distance) => rotate(phrase, -*distance),
            Instruction::RotateBasedOn(ascii_char) => reverse_rotate_based_on(phrase, *ascii_char),
            Instruction::Reverse((x, y)) => reverse_positions(phrase, *x, *y),
            Instruction::Move((x, y)) => move_position(phrase, *y, *x),
        }
    }
}

fn main() {
    let puzzle = puzzle_input_lines("input");

    let instructions: Vec<Instruction> = puzzle
        .map(Result::unwrap)
        .map(|l| Instruction::parse(&l))
        .collect();

    let part_two = Cli::parse_args().part_two;

    let phrase = if part_two { "fbgdceah" } else { "abcdefgh" };

    let mut phrase = phrase.as_ascii().unwrap().to_owned();

    let new_phrase = if part_two {
        for i in instructions.iter().rev() {
            i.run_reverse(&mut phrase);
        }
        phrase.as_str()
    } else {
        for i in instructions {
            i.run(&mut phrase);
        }
        phrase.as_str()
    };
    println!("Phrase: {new_phrase}");
}

fn position_swap(phrase: &mut [ascii::Char], x: usize, y: usize) {
    phrase.swap(x, y);
}

fn letter_swap(phrase: &mut [ascii::Char], letter_x: ascii::Char, letter_y: ascii::Char) {
    let mut pos_x = None;
    let mut pos_y = None;

    for (i, c) in phrase.iter().enumerate() {
        if c == &letter_x {
            pos_x = Some(i);
        } else if c == &letter_y {
            pos_y = Some(i);
        }
    }
    if let (Some(x), Some(y)) = (pos_x, pos_y) {
        position_swap(phrase, x, y);
    }
}

fn rotate(phrase: &mut [ascii::Char], distance: isize) {
    let mut other_phrase = VecDeque::from(phrase.to_owned());
    if distance < 0 {
        for _ in 0..distance.abs() {
            let front = other_phrase.pop_front().unwrap();
            other_phrase.push_back(front);
        }
    } else {
        for _ in 0..distance {
            let back = other_phrase.pop_back().unwrap();
            other_phrase.push_front(back);
        }
    }

    for (i, c) in other_phrase.iter().enumerate() {
        phrase[i] = *c;
    }
}

fn rotate_based_on(phrase: &mut [ascii::Char], letter: ascii::Char) {
    let mut pos = None;

    for (i, c) in phrase.iter().enumerate() {
        if c == &letter {
            pos = Some(i);
        }
    }
    if let Some(mut x) = pos {
        if x >= 4 {
            x += 1;
        }
        x += 1;

        rotate(phrase, x.try_into().unwrap());
    }
}

fn reverse_rotate_based_on(phrase: &mut [ascii::Char], letter: ascii::Char) {
    for i in 1..=phrase.len() {
        let mut new_phrase = phrase.to_vec();
        rotate(&mut new_phrase, i as isize);
        rotate_based_on(&mut new_phrase, letter);
        if new_phrase == phrase {
            rotate(phrase, i as isize);
            break;
        }
    }
}

fn reverse_positions(phrase: &mut [ascii::Char], x: usize, y: usize) {
    let mut sub_phrase = phrase[x..=y].to_vec();
    sub_phrase.reverse();

    for (i, j) in (x..=y).enumerate() {
        phrase[j] = sub_phrase[i];
    }
}

fn move_position(phrase: &mut [ascii::Char], x: usize, y: usize) {
    let mut other_phrase = phrase.to_vec();
    let removed = if x >= other_phrase.len() {
        other_phrase.pop().unwrap()
    } else {
        other_phrase.remove(x)
    };

    if y > other_phrase.len() {
        other_phrase.push(removed);
    } else {
        other_phrase.insert(y, removed);
    }

    for (i, c) in other_phrase.iter().enumerate() {
        phrase[i] = *c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_position_swap() {
        let mut test_string = "abcde".as_ascii().unwrap().to_owned();

        position_swap(&mut test_string, 4, 0);
        assert_eq!(test_string, "ebcda".as_ascii().unwrap());

        position_swap(&mut test_string, 0, 4);
        assert_eq!(test_string, "abcde".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_letter_swap() {
        let mut test_string = "abcde".as_ascii().unwrap().to_owned();

        letter_swap(&mut test_string, ascii::Char::SmallD, ascii::Char::SmallB);
        assert_eq!(test_string, "adcbe".as_ascii().unwrap());

        letter_swap(&mut test_string, ascii::Char::SmallB, ascii::Char::SmallD);
        assert_eq!(test_string, "abcde".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_rotate_left() {
        let mut test_string = "abcd".as_ascii().unwrap().to_owned();

        rotate(&mut test_string, -1);
        assert_eq!(test_string, "bcda".as_ascii().unwrap());

        rotate(&mut test_string, 1);
        assert_eq!(test_string, "abcd".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_rotate_right() {
        let mut test_string = "abcd".as_ascii().unwrap().to_owned();

        rotate(&mut test_string, 1);
        assert_eq!(test_string, "dabc".as_ascii().unwrap());

        rotate(&mut test_string, -1);
        assert_eq!(test_string, "abcd".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_rotate_based_on() {
        let mut test_string = "abcdefgh".as_ascii().unwrap().to_owned();

        rotate_based_on(&mut test_string, ascii::Char::SmallE);
        assert_eq!(test_string, "cdefghab".as_ascii().unwrap());

        reverse_rotate_based_on(&mut test_string, ascii::Char::SmallE);
        assert_eq!(test_string, "abcdefgh".as_ascii().unwrap());
    }

    #[test]
    fn test_reverse_basic_rotate_on_end() {
        let mut test_string = "ecgbhadf".as_ascii().unwrap().to_owned();

        reverse_rotate_based_on(&mut test_string, ascii::Char::SmallF);
        assert_eq!(test_string, "hadfecgb".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_reverse_position() {
        let mut test_string = "abcdefg".as_ascii().unwrap().to_owned();

        reverse_positions(&mut test_string, 1, 4);
        assert_eq!(test_string, "aedcbfg".as_ascii().unwrap());

        reverse_positions(&mut test_string, 1, 4);
        assert_eq!(test_string, "abcdefg".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_move_position_end() {
        let mut test_string = "abcd".as_ascii().unwrap().to_owned();

        move_position(&mut test_string, 1, 4);
        assert_eq!(test_string, "acdb".as_ascii().unwrap());

        move_position(&mut test_string, 4, 1);
        assert_eq!(test_string, "abcd".as_ascii().unwrap());
    }

    #[test]
    fn test_basic_move_position_start() {
        let mut test_string = "abcd".as_ascii().unwrap().to_owned();

        move_position(&mut test_string, 1, 0);
        assert_eq!(test_string, "bacd".as_ascii().unwrap());

        move_position(&mut test_string, 0, 1);
        assert_eq!(test_string, "abcd".as_ascii().unwrap());
    }

    #[test]
    fn test_sample() {
        let mut phrase = "abcde".as_ascii().unwrap().to_vec();
        position_swap(&mut phrase, 4, 0);
        letter_swap(&mut phrase, ascii::Char::SmallD, ascii::Char::SmallB);
        reverse_positions(&mut phrase, 0, 4);
        rotate(&mut phrase, -1);
        move_position(&mut phrase, 1, 4);
        move_position(&mut phrase, 3, 0);
        rotate_based_on(&mut phrase, ascii::Char::SmallB);
        rotate_based_on(&mut phrase, ascii::Char::SmallD);

        assert_eq!(phrase, "decab".as_ascii().unwrap());
    }
}

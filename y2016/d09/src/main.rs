use std::sync::OnceLock;

use aoc_utils::{Cli, get_entire_puzzle};
use regex::Regex;

fn marker_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\((\d*)x(\d*)\)").unwrap())
}

fn main() {
    let puzzle = get_entire_puzzle("input");
    let data = puzzle.first().unwrap();
    let expander = if Cli::parse_args().part_two {
        expand_all
    } else {
        expand
    };
    let expanded = expander(data);

    println!("{}", expanded);
}

fn expand(data: &str) -> usize {
    let re = marker_regex();

    let mut size: usize = 0;
    let mut prev_end: usize = 0;

    for capture in re.captures_iter(data) {
        let length: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let repeat: usize = capture.get(2).unwrap().as_str().parse().unwrap();

        let m = capture.get(0).unwrap();
        let end = m.end();
        if end < prev_end {
            continue;
        }
        size += data[prev_end..m.start()].len();
        size += (data[end..end + length].len()) * repeat;
        prev_end = end + length;
    }

    if prev_end < data.len() {
        size += data[prev_end..].len();
    }

    size
}

fn expand_all(data: &str) -> usize {
    let re = marker_regex();

    let mut size: usize = 0;
    let mut prev_end: usize = 0;

    for capture in re.captures_iter(data) {
        let length: usize = capture.get(1).unwrap().as_str().parse().unwrap();
        let repeat: usize = capture.get(2).unwrap().as_str().parse().unwrap();

        let m = capture.get(0).unwrap();
        let end = m.end();
        if end < prev_end {
            continue;
        }
        size += data[prev_end..m.start()].len();
        size += expand_all(&data[end..end + length]) * repeat;
        prev_end = end + length;
    }

    if prev_end < data.len() {
        size += data[prev_end..].len();
    }

    size
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 6)]
    #[case("X(8x2)(3x3)ABCY", 18)]
    fn part_one_examples(#[case] data: &str, #[case] length: usize) {
        let result = expand(data);

        assert_eq!(result, length);
    }

    #[rstest]
    #[case("(3x3)XYZ", 9)]
    #[case("X(8x2)(3x3)ABCY", 20)]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)]
    fn part_two_examples(#[case] data: &str, #[case] length: usize) {
        let result = expand_all(data);

        assert_eq!(result, length);
    }
}

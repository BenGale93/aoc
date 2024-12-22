use aoc_utils::{puzzle_input_lines, Cli};
use itertools::Itertools;

fn main() {
    let input = puzzle_input_lines("input");
    let is_nice = if Cli::parse_args().part_two {
        is_nice_part_two
    } else {
        is_nice_part_one
    };
    let mut nice = 0;
    for line in input {
        let line = line.unwrap();
        if is_nice(&line) {
            nice += 1;
        }
    }

    dbg!(&nice);
}

fn is_nice_part_one(word: &str) -> bool {
    // condition 3
    if word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy") {
        return false;
    }
    // condition 1
    let mut present_vowels = 0;
    for c in word.chars() {
        if "aeiou".contains(c) {
            present_vowels += 1;
        }
    }
    if present_vowels < 3 {
        return false;
    }
    // condition 2
    let mut success = false;
    for (a, b) in word.chars().tuple_windows() {
        if a == b {
            success = true;
            break;
        }
    }
    success
}

fn is_nice_part_two(word: &str) -> bool {
    let mut condition_two = false;
    for (a, _, c) in word.chars().tuple_windows() {
        if a == c {
            condition_two = true;
        }
    }
    if !condition_two {
        return false;
    }

    let pairs = word.chars().tuple_windows::<(_, _)>();
    let other_pairs = pairs.clone();
    for (i, pair) in pairs.enumerate() {
        for (j, other_pair) in other_pairs.clone().enumerate() {
            if i.abs_diff(j) < 2 {
                continue;
            }
            if pair == other_pair {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_example1() {
        assert!(is_nice_part_one("ugknbfddgicrmopn"))
    }

    #[test]
    fn nice_example2() {
        assert!(is_nice_part_one("aaa"))
    }

    #[test]
    fn not_nice_example1() {
        assert!(!is_nice_part_one("jchzalrnumimnmhp"))
    }

    #[test]
    fn not_nice_example2() {
        assert!(!is_nice_part_one("haegwjzuvuyypxyu"))
    }

    #[test]
    fn not_nice_example3() {
        assert!(!is_nice_part_one("dvszwmarrgswjxmb"))
    }

    #[test]
    fn not_nice_example4() {
        assert!(!is_nice_part_one("aeiouaab"))
    }

    #[test]
    fn nice_part_2_example1() {
        assert!(is_nice_part_two("qjhvhtzxzqqjkmpb"))
    }

    #[test]
    fn nice_part_2_example2() {
        assert!(is_nice_part_two("xxyxx"))
    }

    #[test]
    fn not_nice_part_2_example1() {
        assert!(!is_nice_part_two("uurcxstgmygtbstg"))
    }

    #[test]
    fn not_nice_part_2_example2() {
        assert!(!is_nice_part_two("ieodomkazucvgmuy"))
    }

    #[test]
    fn not_nice_part_2_example3() {
        assert!(!is_nice_part_two("aaa"))
    }
}

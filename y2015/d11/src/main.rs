use std::collections::HashMap;

use itertools::Itertools;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let next_char_map = next_chars();

    let mut current_password = "hepxcrrq".to_string();

    let mut passwords = 0;
    loop {
        current_password = increment(&current_password, &next_char_map);
        if valid_password(&current_password, &next_char_map) {
            println!("Password: {}", current_password);
            passwords += 1;
            if passwords >= 2 {
                break;
            }
        }
    }
}

fn next_chars() -> HashMap<char, char> {
    let mut next_char_map = HashMap::new();
    let chars: Vec<_> = ALPHABET.chars().collect();
    for (a, b) in chars.iter().circular_tuple_windows() {
        next_char_map.insert(*a, *b);
    }
    next_char_map
}

fn requirement_one(input: &str, next_char: &HashMap<char, char>) -> bool {
    let chars: Vec<_> = input.chars().collect();
    for (a, b, c) in chars.iter().tuple_windows() {
        let next_a = next_char.get(a).unwrap();
        if next_a == &'a' || b != next_a {
            continue;
        }
        let next_b = next_char.get(b).unwrap();
        if next_b == &'a' || c != next_b {
            continue;
        }
        return true;
    }
    false
}

fn requirement_two(input: &str) -> bool {
    let result = input.contains('i') || input.contains('o') || input.contains('l');

    !result
}

fn requirement_three(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    for (i, (a, b)) in chars.iter().tuple_windows().enumerate() {
        if a == b {
            for (c, d) in chars.iter().skip(i + 2).tuple_windows() {
                if c == d {
                    return true;
                }
            }
        }
    }
    false
}

fn valid_password(input: &str, next_char: &HashMap<char, char>) -> bool {
    requirement_one(input, next_char) && requirement_two(input) && requirement_three(input)
}

fn increment_char(input: &char, next_char: &HashMap<char, char>) -> char {
    *next_char.get(input).unwrap()
}

fn increment(input: &str, next_char: &HashMap<char, char>) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mut new_chars = chars.clone();
    let num_chars = input.len();
    for (i, c) in chars.iter().rev().enumerate() {
        let next_char = increment_char(c, next_char);
        *new_chars.get_mut(num_chars - i - 1).unwrap() = next_char;
        if next_char != 'a' {
            break;
        }
    }

    String::from_iter(new_chars)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("hijklmmn", true)]
    #[case("abbceffg", false)]
    #[case("yza", false)]
    fn requirement_one_test(#[case] input: &str, #[case] output: bool) {
        let next_char = next_chars();
        assert_eq!(requirement_one(input, &next_char), output)
    }

    #[rstest]
    #[case("aaaaa", true)]
    #[case("aaaaai", false)]
    #[case("aaaaao", false)]
    #[case("aaaaal", false)]
    fn requirement_two_test(#[case] input: &str, #[case] output: bool) {
        assert_eq!(requirement_two(input), output)
    }

    #[rstest]
    #[case("aaaa", true)]
    #[case("aaa", false)]
    #[case("abbceffg", true)]
    #[case("abbcegjk", false)]
    fn requirement_three_test(#[case] input: &str, #[case] output: bool) {
        assert_eq!(requirement_three(input), output)
    }

    #[rstest]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    #[case("hijklmmn", false)]
    fn valid_password_test(#[case] input: &str, #[case] output: bool) {
        let next_char = next_chars();
        assert_eq!(valid_password(input, &next_char), output)
    }

    #[rstest]
    #[case("aa", "ab")]
    #[case("az", "ba")]
    fn increment_test(#[case] input: &str, #[case] output: &str) {
        let next_char = next_chars();
        assert_eq!(increment(input, &next_char), output)
    }
}

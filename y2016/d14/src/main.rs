use aoc_utils::Cli;
use itertools::Itertools;
use memoize::memoize;

#[memoize]
fn get_key(salt: String, index: usize) -> String {
    let data = format!("{salt}{index}");
    let digest = md5::compute(data);
    format!("{:x}", digest)
}

#[memoize]
fn get_stretched_key(salt: String, index: usize) -> String {
    let mut data = format!("{salt}{index}");
    for _ in 0..=2016 {
        let digest = md5::compute(data);
        data = format!("{:x}", digest);
    }
    data
}

fn is_key(salt: &str, index: usize) -> bool {
    let key = get_key(salt.to_string(), index);
    let mut has_triple = false;
    let mut required_char = 'z';
    for (a, b, c) in key.chars().tuple_windows() {
        if a == b && a == c {
            has_triple = true;
            required_char = a;
            break;
        }
    }
    if !has_triple {
        return false;
    }

    for i in 1..=1000 {
        let mut has_five = false;
        let new_key = get_key(salt.to_string(), index + i);
        for (a, b, c, d, e) in new_key.chars().tuple_windows() {
            if a == required_char && a == b && a == c && a == d && a == e {
                has_five = true;
                break;
            }
        }
        if has_five {
            return true;
        }
    }
    false
}

fn is_stretched_key(salt: &str, index: usize) -> bool {
    let key = get_stretched_key(salt.to_string(), index);
    let mut has_triple = false;
    let mut required_char = 'z';
    for (a, b, c) in key.chars().tuple_windows() {
        if a == b && a == c {
            has_triple = true;
            required_char = a;
            break;
        }
    }
    if !has_triple {
        return false;
    }

    for i in 1..=1000 {
        let mut has_five = false;
        let new_key = get_stretched_key(salt.to_string(), index + i);
        for (a, b, c, d, e) in new_key.chars().tuple_windows() {
            if a == required_char && a == b && a == c && a == d && a == e {
                has_five = true;
                break;
            }
        }
        if has_five {
            return true;
        }
    }
    false
}

fn main() {
    let salt = "zpqevtbw";

    let mut count = 0;
    let mut index = 0;
    let checker = if Cli::parse_args().part_two {
        is_stretched_key
    } else {
        is_key
    };

    loop {
        if checker(salt, index) {
            count += 1;
        }
        if count == 64 {
            break;
        }
        index += 1;
    }
    println!("{index}")
}

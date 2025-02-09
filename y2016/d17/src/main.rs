use std::collections::VecDeque;

use aoc_utils::{Direction, out_of_bounds};

fn is_open(c: &char) -> bool {
    *c == 'b' || *c == 'c' || *c == 'd' || *c == 'e' || *c == 'f'
}

fn hash(passcode: &str, path: &str) -> Vec<char> {
    let digest = md5::compute(format!("{passcode}{path}"));
    let hash = format!("{:x}", digest);
    hash.chars().take(4).collect()
}

fn main() {
    let passcode = "njfxhljp";
    let size = 4;
    let mut paths = VecDeque::new();
    paths.push_back(((0, 0), "".to_string()));
    let mut max_length = 0;
    let mut shortest_found = false;
    while let Some((coord, path)) = paths.pop_front() {
        if coord == (3, 3) {
            if !shortest_found {
                println!("{path}");
                shortest_found = true;
            }
            if path.len() >= max_length {
                max_length = path.len();
            }
            continue;
        }
        let hash_chars = hash(passcode, &path);
        if is_open(&hash_chars[0]) {
            let next_coord = Direction::Up.next_coord(&coord);
            if !out_of_bounds(&next_coord, size) {
                paths.push_back((next_coord, format!("{path}U")));
            }
        }
        if is_open(&hash_chars[1]) {
            let next_coord = Direction::Down.next_coord(&coord);
            if !out_of_bounds(&next_coord, size) {
                paths.push_back((next_coord, format!("{path}D")));
            }
        }
        if is_open(&hash_chars[2]) {
            let next_coord = Direction::Left.next_coord(&coord);
            if !out_of_bounds(&next_coord, size) {
                paths.push_back((next_coord, format!("{path}L")));
            }
        }
        if is_open(&hash_chars[3]) {
            let next_coord = Direction::Right.next_coord(&coord);
            if !out_of_bounds(&next_coord, size) {
                paths.push_back((next_coord, format!("{path}R")));
            }
        }
    }
    println!("{max_length}");
}

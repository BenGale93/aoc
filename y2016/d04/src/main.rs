use std::{cmp::Ordering, collections::HashMap};

use aoc_utils::{Cli, FromRegex, parse_with_regex};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone)]
struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl FromRegex for Room {
    fn from_regex(line: &str, re: &regex::Regex) -> Self {
        let capture = re.captures(line).unwrap();
        Room {
            name: capture.get(1).unwrap().as_str().to_string(),
            id: capture.get(2).unwrap().as_str().parse().unwrap(),
            checksum: capture.get(3).unwrap().as_str().to_string(),
        }
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let mut counts = HashMap::new();

        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut ordered_counts: Vec<_> = counts.iter().collect();
        ordered_counts.sort_by(|a, b| {
            let cmp = b.1.cmp(a.1);
            if cmp != Ordering::Equal {
                return cmp;
            }
            a.0.cmp(b.0)
        });

        for (check, (count, _)) in self.checksum.chars().zip(ordered_counts) {
            if check != *count {
                return false;
            }
        }

        true
    }

    fn decrypt(&self, alphabet: &[char]) -> String {
        let advance = self.id % 26;
        let mut real_name: Vec<char> = vec![];
        for c in self.name.chars() {
            if c == '-' {
                real_name.push(' ');
                continue;
            }
            let position = alphabet.iter().position(|&a| a == c).unwrap();
            let new_position = (position + advance) % 26;
            let new_letter = alphabet.get(new_position).unwrap();
            real_name.push(*new_letter);
        }

        real_name.iter().collect()
    }
}

fn part_two(valid_rooms: &[Room]) -> usize {
    let alphabet: Vec<_> = ALPHABET.chars().collect();
    for room in valid_rooms {
        let real_name = room.decrypt(&alphabet);
        if real_name == "northpole object storage " {
            return room.id;
        }
    }
    panic!()
}

fn main() {
    let re = regex::Regex::new(r"([a-z-]+)(\d+)\[([a-z]+)\]").unwrap();

    let rooms: Vec<Room> = parse_with_regex("input", re);

    let valid_rooms: Vec<_> = rooms.into_iter().filter(|r| r.is_real()).collect();

    let result: usize = if Cli::parse_args().part_two {
        part_two(&valid_rooms)
    } else {
        valid_rooms.iter().map(|r| r.id).sum()
    };
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_one() {
        let room = Room {
            name: "aaaaabbbzyx".to_string(),
            id: 123,
            checksum: "abxyz".to_string(),
        };
        assert!(room.is_real());
    }

    #[test]
    fn part_one_example_four() {
        let room = Room {
            name: "totallyrealroom".to_string(),
            id: 200,
            checksum: "decoy".to_string(),
        };
        assert!(!room.is_real());
    }

    #[test]
    fn part_two_example_one() {
        let room = Room {
            name: "qzmt-zixmtkozy-ivhz".to_string(),
            id: 343,
            checksum: "zidch".to_string(),
        };

        let alphabet: Vec<_> = ALPHABET.chars().collect();
        assert_eq!(room.decrypt(&alphabet), "very encrypted name".to_string());
    }
}

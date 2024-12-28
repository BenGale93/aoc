use std::collections::{HashMap, HashSet};

use aoc_utils::puzzle_input_lines;
use itertools::Itertools;

fn main() {
    let lines = puzzle_input_lines("input");

    let mut valid_places = HashSet::new();
    let mut distance_map = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let line_elements: Vec<&str> = line.split('=').collect();
        let distance = line_elements.last().unwrap();
        let distance: usize = distance.trim().parse().unwrap();

        let places: Vec<_> = line_elements.first().unwrap().split("to").collect();
        let start = places.first().unwrap().trim();
        let end = places.last().unwrap().trim();
        distance_map.insert((start.to_string(), end.to_string()), distance);
        distance_map.insert((end.to_string(), start.to_string()), distance);
        valid_places.insert(start.to_string());
        valid_places.insert(end.to_string());
    }

    let mut longest = 0;
    let mut shortest = usize::MAX;
    for place_combo in valid_places.iter().permutations(valid_places.len()) {
        let mut total_distance = 0;
        for (start, end) in place_combo.into_iter().tuple_windows() {
            let distance = distance_map
                .get(&(start.to_string(), end.to_string()))
                .unwrap();
            total_distance += distance;
        }
        if total_distance < shortest {
            shortest = total_distance;
        }
        if total_distance > longest {
            longest = total_distance;
        }
    }

    println!("Shortest: {}", shortest);
    println!("Longest: {}", longest);
}

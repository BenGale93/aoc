use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

use aoc_utils::{Cli, puzzle_input_lines};
use regex::Regex;

fn main() {
    let test_case = [
        "children: 3",
        "cats: 7",
        "samoyeds: 2",
        "pomeranians: 3",
        "akitas: 0",
        "vizslas: 0",
        "goldfish: 5",
        "trees: 3",
        "cars: 2",
        "perfumes: 1",
    ];
    let lines = puzzle_input_lines("input");

    let sue = if Cli::parse_args().part_two {
        compute_sue_part_two(lines, &test_case)
    } else {
        compute_sue_part_one(lines, &test_case)
    };

    println!("{}", sue);
}

fn compute_sue_part_one(lines: Lines<BufReader<File>>, test_case: &[&str]) -> usize {
    let categories: Vec<_> = test_case
        .iter()
        .map(|s| (s.split(':').map(|s| s.trim()).collect::<Vec<_>>(), s))
        .map(|(v, s)| (*v.first().unwrap(), s))
        .collect();
    let mut sue = 1;
    for line in lines {
        let line = line.unwrap();
        let valid = categories
            .iter()
            .all(|(cat, full_cat)| !line.contains(cat) || line.contains(*full_cat));
        if valid {
            return sue;
        }
        sue += 1;
    }
    usize::MAX
}

fn get_count(line: &str, re: &Regex) -> usize {
    re.captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn compute_sue_part_two(lines: Lines<BufReader<File>>, test_case: &[&str]) -> usize {
    let re_map: HashMap<_, _> = HashMap::from_iter([
        ("cats", regex::Regex::new(r"cats: (\d*)").unwrap()),
        ("trees", regex::Regex::new(r"trees: (\d*)").unwrap()),
        (
            "pomeranians",
            regex::Regex::new(r"pomeranians: (\d*)").unwrap(),
        ),
        ("goldfish", regex::Regex::new(r"goldfish: (\d*)").unwrap()),
    ]);

    let categories: Vec<_> = test_case
        .iter()
        .map(|s| (s.split(':').map(|s| s.trim()).collect::<Vec<_>>(), s))
        .map(|(v, s)| {
            (
                *v.first().unwrap(),
                v.last().unwrap().parse::<usize>().unwrap(),
                s,
            )
        })
        .collect();

    let mut sue = 1;
    for line in lines {
        let line = line.unwrap();
        let mut valid: Vec<bool> = vec![];
        for (cat, count, full_cat) in &categories {
            if !line.contains(cat) {
                valid.push(true)
            } else if *cat == "cats" || *cat == "trees" {
                let actual_count = get_count(&line, re_map.get(cat).unwrap());
                if actual_count > *count {
                    valid.push(true)
                } else {
                    valid.push(false)
                }
            } else if *cat == "pomeranians" || *cat == "goldfish" {
                let actual_count = get_count(&line, re_map.get(cat).unwrap());
                if actual_count < *count {
                    valid.push(true)
                } else {
                    valid.push(false)
                }
            } else {
                valid.push(line.contains(**full_cat))
            }
        }
        if valid.iter().all(|x| *x) {
            return sue;
        }
        sue += 1;
    }
    usize::MAX
}

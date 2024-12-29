use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use aoc_utils::{Cli, puzzle_input_lines};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let lines = puzzle_input_lines("input");
    let re = Regex::new(
        r"([a-zA-Z]*) would (gain|lose) (\d*) happiness units by sitting next to ([a-zA-Z]*).",
    )
    .unwrap();

    let mut persons = HashSet::new();
    if Cli::parse_args().part_two {
        persons.insert(Rc::new("me".to_string()));
    };
    let mut happiness_map: HashMap<(Rc<_>, Rc<_>), i64> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let (person, next_person, units) = parse_line(&line, &re);
        let person = Rc::new(person);
        persons.insert(person.clone());
        happiness_map.insert((person, Rc::new(next_person)), units);
    }

    let total_persons = persons.len();
    let max_happiness = persons
        .iter()
        .permutations(total_persons)
        .map(|perm| {
            perm.into_iter()
                .circular_tuple_windows()
                .map(|(person, next_person)| {
                    happiness_map
                        .get(&(person.clone(), next_person.clone()))
                        .unwrap_or(&0)
                        + happiness_map
                            .get(&(next_person.clone(), person.clone()))
                            .unwrap_or(&0)
                })
                .sum::<i64>()
        })
        .max()
        .unwrap();

    println!("{}", max_happiness);
}

fn parse_line(line: &str, re: &Regex) -> (String, String, i64) {
    let capture = re.captures(line).unwrap();
    let unsigned_units: u64 = capture.get(3).unwrap().as_str().parse().unwrap();
    let gain = capture.get(2).unwrap().as_str() == "gain";
    let units = if gain {
        unsigned_units as i64
    } else {
        -(unsigned_units as i64)
    };
    (
        capture.get(1).unwrap().as_str().to_string(),
        capture.get(4).unwrap().as_str().to_string(),
        units,
    )
}

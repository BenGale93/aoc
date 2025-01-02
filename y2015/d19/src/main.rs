use std::collections::HashSet;

use aoc_utils::{Cli, get_entire_puzzle};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let puzzle = get_entire_puzzle("input");

    let medicine = puzzle.last().unwrap();

    let transitions: Vec<(_, _)> = puzzle[0..(puzzle.len() - 2)]
        .iter()
        .map(|s| {
            let split: Vec<_> = s.split(" => ").collect();
            (
                split.first().unwrap().to_string(),
                split.last().unwrap().to_string(),
            )
        })
        .collect();

    let result = if Cli::parse_args().part_two {
        backtrack_molecule(medicine, &transitions)
    } else {
        let new_molecules = get_new_molecules(medicine, &transitions);
        new_molecules.len()
    };

    println!("Total: {}", result);
}

fn get_new_molecules(initial_molecule: &str, transitions: &[(String, String)]) -> HashSet<String> {
    let mut new_molecules = HashSet::new();
    for (before, after) in transitions {
        for i in 0..initial_molecule.len() {
            let range = i..i + before.len();
            let text_range = initial_molecule.get(range.clone());
            if text_range.is_some_and(|t| t == *before) {
                let mut new_molecule = initial_molecule.to_string();
                new_molecule.replace_range(range, after);
                new_molecules.insert(new_molecule);
            }
        }
    }
    new_molecules
}

fn backtrack_molecule(initial_molecule: &str, transitions: &[(String, String)]) -> usize {
    let mut rng = thread_rng();
    let mut transitions_rand = transitions.to_vec();
    let mut steps = 0;

    let mut molecule = initial_molecule.to_string();

    while molecule != *"e" {
        let tmp_molecule = molecule.clone();
        for (after, before) in &transitions_rand {
            let location = molecule.find(before);
            if let Some(l) = location {
                molecule.replace_range(l..l + before.len(), after);
                steps += 1;
                break;
            }
        }
        if tmp_molecule == molecule {
            molecule = initial_molecule.to_string();
            steps = 0;
            transitions_rand.shuffle(&mut rng);
        }
    }

    steps
}

use std::collections::{HashSet, VecDeque};

use aoc_utils::Cli;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Variant {
    Chip,
    Generator,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Machine {
    name: u8,
    variant: Variant,
    floor: usize,
}

impl Machine {
    fn new(name: u8, variant: Variant, floor: usize) -> Self {
        Self {
            name,
            variant,
            floor,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Scenario {
    machines: Vec<Machine>,
    floor: usize,
}

impl Scenario {
    fn is_valid(&self) -> bool {
        let mut valids = vec![];
        for i in 0..4 {
            let chips: Vec<_> = self
                .machines
                .iter()
                .filter(|m| m.floor == i && matches!(m.variant, Variant::Chip))
                .collect();
            let generators: Vec<_> = self
                .machines
                .iter()
                .filter(|m| m.floor == i && matches!(m.variant, Variant::Generator))
                .collect();
            if generators.is_empty() || chips.is_empty() {
                valids.push(true);
                continue;
            }

            let matching_generators = chips
                .iter()
                .all(|c| generators.iter().any(|g| g.name == c.name));
            valids.push(matching_generators);
        }
        valids.iter().all(|b| *b)
    }

    fn is_done(&self) -> bool {
        self.machines.iter().all(|m| m.floor == 3)
    }

    fn move_machines(&self) -> Vec<Self> {
        let machines_indexes: Vec<_> = self
            .machines
            .iter()
            .enumerate()
            .filter(|(_, m)| m.floor == self.floor)
            .map(|(i, _)| i)
            .collect();
        let mut up_scenarios = vec![];
        let mut down_scenarios = vec![];
        let is_below_empty = self
            .machines
            .iter()
            .filter(|f| f.floor < self.floor)
            .count()
            == 0;
        for i in &machines_indexes {
            if self.floor < 3 {
                let mut up_scenario = self.clone();
                up_scenario.machines[*i].floor += 1;
                up_scenario.floor += 1;
                if up_scenario.is_valid() {
                    up_scenarios.push(up_scenario);
                }
            }

            if !is_below_empty && self.floor > 0 {
                let mut down_scenario = self.clone();
                down_scenario.machines[*i].floor -= 1;
                down_scenario.floor -= 1;
                if down_scenario.is_valid() {
                    down_scenarios.push(down_scenario);
                }
            }
        }
        let mut up_clear = false;
        for (i, j) in machines_indexes.iter().tuple_combinations() {
            if self.floor < 3 {
                let mut up_scenario = self.clone();
                up_scenario.machines[*i].floor += 1;
                up_scenario.machines[*j].floor += 1;
                up_scenario.floor += 1;
                if up_scenario.is_valid() {
                    if !up_scenarios.is_empty() && !up_clear {
                        up_scenarios.clear();
                        up_clear = true;
                    }
                    up_scenarios.push(up_scenario);
                }
            }

            if !down_scenarios.is_empty() || (!is_below_empty && self.floor > 0) {
                let mut down_scenario = self.clone();
                down_scenario.machines[*i].floor -= 1;
                down_scenario.machines[*j].floor -= 1;
                down_scenario.floor -= 1;
                if down_scenario.is_valid() {
                    down_scenarios.push(down_scenario);
                }
            }
        }

        up_scenarios.into_iter().chain(down_scenarios).collect()
    }
}

fn main() {
    let mut start_machines = vec![
        Machine::new(0, Variant::Chip, 0),
        Machine::new(0, Variant::Generator, 0),
        Machine::new(1, Variant::Generator, 1),
        Machine::new(2, Variant::Generator, 1),
        Machine::new(3, Variant::Generator, 1),
        Machine::new(4, Variant::Generator, 1),
        Machine::new(1, Variant::Chip, 2),
        Machine::new(2, Variant::Chip, 2),
        Machine::new(3, Variant::Chip, 2),
        Machine::new(4, Variant::Chip, 2),
    ];
    if Cli::parse_args().part_two {
        start_machines.push(Machine::new(5, Variant::Chip, 0));
        start_machines.push(Machine::new(5, Variant::Generator, 0));
        start_machines.push(Machine::new(6, Variant::Chip, 0));
        start_machines.push(Machine::new(6, Variant::Generator, 0));
    };
    let start_scenario = Scenario {
        machines: start_machines,
        floor: 0,
    };

    let mut scenarios = VecDeque::new();
    scenarios.push_back((start_scenario.clone(), 0));
    let mut seen = HashSet::new();
    seen.insert(start_scenario);

    while let Some((scenario, steps)) = scenarios.pop_front() {
        assert!(scenario.is_valid());
        if scenario.is_done() {
            println!("Steps {}", steps);
            break;
        }

        for new_scenario in scenario.move_machines() {
            if !seen.contains(&new_scenario) {
                scenarios.push_back((new_scenario.clone(), steps + 1));
                seen.insert(new_scenario);
            }
        }
    }
}

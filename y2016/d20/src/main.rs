use aoc_utils::{Cli, puzzle_input_lines};

fn main() {
    let lines = puzzle_input_lines("input");

    let mut firewall = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let numbers = line.split_once('-').unwrap();
        let numbers: (i64, i64) = (numbers.0.parse().unwrap(), numbers.1.parse().unwrap());
        assert!(numbers.0 < numbers.1);
        firewall.push(numbers);
    }

    firewall.sort_by(|a, b| a.0.cmp(&b.0));

    if Cli::parse_args().part_two {
        let mut values = Vec::new();
        let mut value = 0;
        let mut i = 0;
        while i < firewall.len() {
            let numbers = firewall[i];
            let range = numbers.0..=numbers.1;
            if range.contains(&value) {
                value = numbers.1 + 1;
                i += 1;
            } else if value > numbers.1 {
                i += 1;
            } else {
                values.push(value);
                value += 1;
            }
        }
        println!("Length: {}", values.len());
    } else {
        let mut value = 0;
        for numbers in firewall {
            let range = numbers.0..=numbers.1;
            if range.contains(&value) {
                value = numbers.1 + 1;
            } else if value > numbers.1 {
                continue;
            } else {
                break;
            }
        }
        println!("Value {value}");
    }
}

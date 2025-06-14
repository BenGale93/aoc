use std::sync::OnceLock;

use aoc_utils::{Cli, puzzle_input_lines};
use regex::Regex;

fn node_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"\/dev\/grid\/node-x(\d*)-y(\d*) *(\d*)T *(\d*)T *(\d*)T *(\d*)%").unwrap()
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: u8,
    y: u8,
    size: usize,
    used: usize,
    available: usize,
}

impl Node {
    fn parse(input: &str) -> Self {
        if let Some(capture) = node_regex().captures(input) {
            return Self {
                x: capture[1].parse().unwrap(),
                y: capture[2].parse().unwrap(),
                size: capture[3].parse().unwrap(),
                used: capture[4].parse().unwrap(),
                available: capture[5].parse().unwrap(),
            };
        }
        panic!("Could not parse");
    }
}

fn main() {
    let puzzle = puzzle_input_lines("input");

    let nodes: Vec<Node> = puzzle
        .map(Result::unwrap)
        .skip(2)
        .map(|l| Node::parse(&l))
        .collect();

    if Cli::parse_args().part_two {
        let mut x0 = None;
        let mut y0 = None;
        let mut xmax = 0;
        for node in nodes {
            if node.used == 0 {
                x0 = Some(node.x);
                y0 = Some(node.y);
            }
            if node.x > xmax {
                xmax = node.x;
            }
        }
        if let (Some(x), Some(y)) = (x0, y0) {
            let value = x + y + xmax + (xmax - 1) * 5;
            println!("Value {value}");
        }
    } else {
        let mut viable = 0;
        for node_a in &nodes {
            if node_a.used == 0 {
                continue;
            }
            for node_b in &nodes {
                if node_a == node_b {
                    continue;
                }
                if node_a.used <= node_b.available {
                    viable += 1;
                }
            }
        }
        println!("Viable {viable}");
    }
}

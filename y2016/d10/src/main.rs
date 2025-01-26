use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

use aoc_utils::{FromRegex, get_entire_puzzle};
use regex::Regex;

fn start_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"value (\d*) goes to bot (\d*)").unwrap())
}

fn gives_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)")
            .unwrap()
    })
}

#[derive(Debug, Clone, Copy)]
struct Start {
    value: usize,
    bot: Destination,
}

impl FromRegex for Start {
    fn from_regex(line: &str, re: &regex::Regex) -> Self {
        let capture = re.captures(line).unwrap();

        Self {
            value: capture.get(1).unwrap().as_str().parse().unwrap(),
            bot: Destination::Bot(capture.get(2).unwrap().as_str().parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl Destination {
    fn id(&self) -> usize {
        match self {
            Destination::Bot(i) => *i,
            Destination::Output(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Give {
    start: Destination,
    low: Destination,
    high: Destination,
}

impl FromRegex for Give {
    fn from_regex(line: &str, re: &regex::Regex) -> Self {
        let capture = re.captures(line).unwrap();

        let low = if capture.get(2).unwrap().as_str().contains("output") {
            Destination::Output(capture.get(3).unwrap().as_str().parse().unwrap())
        } else {
            Destination::Bot(capture.get(3).unwrap().as_str().parse().unwrap())
        };

        let high = if capture.get(4).unwrap().as_str().contains("output") {
            Destination::Output(capture.get(5).unwrap().as_str().parse().unwrap())
        } else {
            Destination::Bot(capture.get(5).unwrap().as_str().parse().unwrap())
        };

        Self {
            start: Destination::Bot(capture.get(1).unwrap().as_str().parse().unwrap()),
            low,
            high,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl Bot {
    fn new(id: usize) -> Self {
        Self {
            id,
            left: None,
            right: None,
        }
    }

    fn give(&mut self, value: usize) -> bool {
        if self.left.is_none() {
            self.left = Some(value);
            false
        } else if self.right.is_none() {
            self.right = Some(value);
            true
        } else {
            unreachable!()
        }
    }

    fn compare(&self) -> (usize, usize) {
        let (Some(left), Some(right)) = (self.left, self.right) else {
            unreachable!();
        };
        if left < right {
            (left, right)
        } else {
            (right, left)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Output {
    zero: Option<usize>,
    one: Option<usize>,
    two: Option<usize>,
}

impl Output {
    fn new() -> Self {
        Self {
            zero: None,
            one: None,
            two: None,
        }
    }

    fn full(&self) -> bool {
        self.zero.is_some() && self.one.is_some() && self.two.is_some()
    }

    fn value(&self) -> usize {
        self.zero.unwrap() * self.one.unwrap() * self.two.unwrap()
    }

    fn populate(&mut self, out_id: usize, value: usize) {
        if out_id == 0 {
            self.zero = Some(value);
        } else if out_id == 1 {
            self.one = Some(value);
        } else if out_id == 2 {
            self.two = Some(value);
        }
    }
}

fn main() {
    let low_compare = 17;
    let high_compare = 61;
    let puzzle = get_entire_puzzle("input");

    let starts: Vec<_> = puzzle
        .iter()
        .filter(|l| l.contains("value"))
        .map(|l| Start::from_regex(l, start_regex()))
        .collect();

    let gives: HashMap<_, _> = puzzle
        .iter()
        .filter(|l| l.contains("high"))
        .map(|l| Give::from_regex(l, gives_regex()))
        .map(|g| (g.start.id(), g))
        .collect();

    let mut active_bots: HashMap<usize, Bot> = HashMap::new();
    let mut has_two = VecDeque::new();
    for s in starts {
        let bot_id = s.bot.id();
        let bot = active_bots.entry(bot_id).or_insert(Bot::new(bot_id));
        let full = bot.give(s.value);
        if full {
            has_two.push_back(bot_id);
        }
    }

    let mut output = Output::new();
    while let Some(bot_id) = has_two.pop_front() {
        let bot = active_bots
            .remove(&bot_id)
            .expect("expected the bot to exist");
        let instruction = gives
            .get(&bot.id)
            .expect("expected the bot to exist in the instructions");
        let (low, high) = bot.compare();
        if low == low_compare && high == high_compare {
            println!("bot {}", bot.id);
        }
        match instruction.low {
            Destination::Bot(low_id) => {
                let low_bot = active_bots.entry(low_id).or_insert(Bot::new(low_id));
                let full = low_bot.give(low);
                if full {
                    has_two.push_back(low_id);
                }
            }
            Destination::Output(out_id) => output.populate(out_id, low),
        };
        match instruction.high {
            Destination::Bot(high_id) => {
                let high_bot = active_bots.entry(high_id).or_insert(Bot::new(high_id));
                let full = high_bot.give(high);
                if full {
                    has_two.push_back(high_id);
                }
            }
            Destination::Output(out_id) => output.populate(out_id, high),
        };

        if output.full() {
            println!("Final value {}", output.value());
            break;
        }
    }
}

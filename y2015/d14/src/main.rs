use aoc_utils::{FromRegex, parse_with_regex};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Status {
    Fly,
    Rest,
}

impl Status {
    fn flip(self) -> Self {
        match self {
            Self::Fly => Self::Rest,
            Self::Rest => Self::Fly,
        }
    }
}

#[derive(Debug)]
struct Reindeer {
    speed: usize,
    fly: usize,
    rest: usize,
    distance: usize,
    status: Status,
    tick: usize,
    score: usize,
}

impl Reindeer {
    fn move_deer(&mut self) {
        self.tick += 1;
        match self.status {
            Status::Fly => {
                if self.tick > self.fly {
                    self.status = self.status.flip();
                    self.tick = 1;
                } else {
                    self.distance += self.speed;
                }
            }
            Status::Rest => {
                if self.tick >= self.rest {
                    self.status = self.status.flip();
                    self.tick = 0;
                }
            }
        }
    }
}

impl FromRegex for Reindeer {
    fn from_regex(line: &str, re: &regex::Regex) -> Self {
        let capture = re.captures(line).unwrap();

        Reindeer {
            speed: capture.get(1).unwrap().as_str().parse().unwrap(),
            fly: capture.get(2).unwrap().as_str().parse().unwrap(),
            rest: capture.get(3).unwrap().as_str().parse().unwrap(),
            distance: 0,
            status: Status::Fly,
            tick: 0,
            score: 0,
        }
    }
}

fn main() {
    let re = Regex::new(r"(\d*)\D*(\d*) seconds, but then must rest for (\d*)").unwrap();

    let mut reindeers: Vec<Reindeer> = parse_with_regex("input", re);

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            reindeer.move_deer();
        }
        let max_distance = reindeers.iter().map(|r| r.distance).max().unwrap();
        for r in &mut reindeers {
            if r.distance == max_distance {
                r.score += 1;
            }
        }
    }

    let top_distance = reindeers.iter().map(|r| r.distance).max().unwrap();
    let top_score = reindeers.iter().map(|r| r.score).max().unwrap();

    println!("Top distance: {top_distance}");
    println!("Top score: {top_score}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example_one() {
        let mut r = Reindeer {
            speed: 14,
            fly: 10,
            rest: 127,
            distance: 0,
            status: Status::Fly,
            tick: 0,
            score: 0,
        };

        for _ in 0..1000 {
            r.move_deer();
        }

        assert_eq!(r.distance, 1120);
    }

    #[test]
    fn part_one_example_two() {
        let mut r = Reindeer {
            speed: 16,
            fly: 11,
            rest: 162,
            distance: 0,
            status: Status::Fly,
            tick: 0,
            score: 0,
        };

        for _ in 0..1000 {
            r.move_deer();
        }

        assert_eq!(r.distance, 1056);
    }
}

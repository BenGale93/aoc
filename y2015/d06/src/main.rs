use aoc_utils::{puzzle_input_lines, Cli};
use regex::Regex;

#[derive(Clone, Copy)]
enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Op {
    fn adjust(self, brightness: usize) -> usize {
        match self {
            Op::TurnOn => brightness + 1,
            Op::TurnOff => brightness.saturating_sub(1),
            Op::Toggle => brightness + 2,
        }
    }
}

#[derive(Default, Clone, Copy)]
enum Light {
    #[default]
    Off,
    On,
}

impl Light {
    fn switch(self, op: Op) -> Self {
        match (self, op) {
            (Light::Off, Op::TurnOn) => Light::On,
            (Light::Off, Op::TurnOff) => Light::Off,
            (Light::Off, Op::Toggle) => Light::On,
            (Light::On, Op::TurnOn) => Light::On,
            (Light::On, Op::TurnOff) => Light::Off,
            (Light::On, Op::Toggle) => Light::Off,
        }
    }
}

type Grid = [[Light; 1000]; 1000];

fn follow_instruction(grid: &mut Grid, op: Op, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            let light = grid[x][y];
            grid[x][y] = light.switch(op);
        }
    }
}

fn part_one() -> usize {
    let re = Regex::new(r"(turn off|turn on|toggle) (\d*),(\d*) through (\d*),(\d*)").unwrap();
    let input = puzzle_input_lines("input");
    let mut grid = [[Light::Off; 1000]; 1000];

    for line in input {
        let line = line.unwrap();
        for (_, [op_str, x1, y1, x2, y2]) in re.captures_iter(&line).map(|caps| caps.extract()) {
            let op = match op_str {
                "turn on" => Op::TurnOn,
                "turn off" => Op::TurnOff,
                "toggle" => Op::Toggle,
                _ => panic!(),
            };
            follow_instruction(
                &mut grid,
                op,
                x1.parse().unwrap(),
                y1.parse().unwrap(),
                x2.parse().unwrap(),
                y2.parse().unwrap(),
            );
        }
    }

    let mut lit = 0;
    for y in grid {
        for x in y {
            if matches!(x, Light::On) {
                lit += 1;
            }
        }
    }
    lit
}

fn follow_instruction_two(
    grid: &mut [Vec<usize>],
    op: Op,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            let light = grid[x][y];
            grid[x][y] = op.adjust(light);
        }
    }
}

fn part_two() -> usize {
    let re = Regex::new(r"(turn off|turn on|toggle) (\d*),(\d*) through (\d*),(\d*)").unwrap();
    let input = puzzle_input_lines("input");
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for line in input {
        let line = line.unwrap();
        for (_, [op_str, x1, y1, x2, y2]) in re.captures_iter(&line).map(|caps| caps.extract()) {
            let op = match op_str {
                "turn on" => Op::TurnOn,
                "turn off" => Op::TurnOff,
                "toggle" => Op::Toggle,
                _ => panic!(),
            };
            follow_instruction_two(
                &mut grid,
                op,
                x1.parse().unwrap(),
                y1.parse().unwrap(),
                x2.parse().unwrap(),
                y2.parse().unwrap(),
            );
        }
    }

    let mut brightness = 0;
    for y in grid {
        for x in y {
            brightness += x;
        }
    }
    brightness
}

fn main() {
    let result = if Cli::parse_args().part_two {
        part_two()
    } else {
        part_one()
    };

    println!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two_example1() {
        let mut grid = vec![vec![0]];
        follow_instruction_two(&mut grid, Op::TurnOn, 0, 0, 0, 0);

        let mut brightness = 0;
        for y in grid {
            for x in y {
                brightness += x;
            }
        }
        assert_eq!(brightness, 1);
    }

    #[test]
    fn part_two_example2() {
        let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
        follow_instruction_two(&mut grid, Op::Toggle, 0, 0, 999, 999);

        let mut brightness = 0;
        for y in grid {
            for x in y {
                brightness += x;
            }
        }
        assert_eq!(brightness, 2000000);
    }

    #[test]
    fn part_two_example3() {
        let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
        follow_instruction_two(&mut grid, Op::Toggle, 0, 0, 999, 999);
        follow_instruction_two(&mut grid, Op::TurnOff, 0, 0, 999, 999);

        let mut brightness = 0;
        for y in grid {
            for x in y {
                brightness += x;
            }
        }
        assert_eq!(brightness, 1000000);
    }
}

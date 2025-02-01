use aoc_utils::{Cli, puzzle_input_lines};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Inst {
    Cpy((isize, usize)),
    CpyOther((usize, usize)),
    Inc(usize),
    Dec(usize),
    Jnz((usize, isize)),
    JnzOther((isize, isize)),
}

fn reg_to_index(reg: &str) -> usize {
    match reg {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => unreachable!(),
    }
}

fn main() {
    let puzzle = puzzle_input_lines("input");
    let cpy_reg = Regex::new(r"cpy (\d*) ([abcd])").unwrap();
    let cpy_other_reg = Regex::new(r"cpy ([abcd]) ([abcd])").unwrap();
    let inc_reg = Regex::new(r"inc ([abcd])").unwrap();
    let dec_reg = Regex::new(r"dec ([abcd])").unwrap();
    let jnz_reg = Regex::new(r"jnz ([abcd]) (-?\d*)").unwrap();
    let jnz_other_reg = Regex::new(r"jnz (\d*) (-?\d*)").unwrap();

    let mut instructions = vec![];
    for line in puzzle {
        let line = line.unwrap();
        if let Some(m) = cpy_reg.captures(&line) {
            let reg_index = reg_to_index(&m[2]);
            instructions.push(Inst::Cpy((m[1].parse().unwrap(), reg_index)));
        } else if let Some(m) = cpy_other_reg.captures(&line) {
            let start_index = reg_to_index(&m[1]);
            let end_index = reg_to_index(&m[2]);
            instructions.push(Inst::CpyOther((start_index, end_index)));
        } else if let Some(m) = inc_reg.captures(&line) {
            instructions.push(Inst::Inc(reg_to_index(&m[1])));
        } else if let Some(m) = dec_reg.captures(&line) {
            instructions.push(Inst::Dec(reg_to_index(&m[1])));
        } else if let Some(m) = jnz_reg.captures(&line) {
            let reg_index = reg_to_index(&m[1]);
            instructions.push(Inst::Jnz((reg_index, m[2].parse().unwrap())));
        } else if let Some(m) = jnz_other_reg.captures(&line) {
            instructions.push(Inst::JnzOther((
                m[1].parse().unwrap(),
                m[2].parse().unwrap(),
            )));
        }
    }

    assert_eq!(instructions.len(), 23);

    let mut registers = if Cli::parse_args().part_two {
        [0, 0, 1, 0]
    } else {
        [0, 0, 0, 0]
    };

    let mut position: isize = 0;
    while let Some(i) = instructions.get(position as usize) {
        match i {
            Inst::Cpy((v, r)) => {
                if let Some(elem) = registers.get_mut(*r) {
                    *elem = *v;
                }
                position += 1;
            }
            Inst::CpyOther((s, e)) => {
                let start_value = registers[*s];
                if let Some(end) = registers.get_mut(*e) {
                    *end = start_value;
                }
                position += 1;
            }
            Inst::Inc(r) => {
                if let Some(elem) = registers.get_mut(*r) {
                    *elem += 1;
                }
                position += 1;
            }
            Inst::Dec(r) => {
                if let Some(elem) = registers.get_mut(*r) {
                    *elem -= 1;
                }
                position += 1;
            }
            Inst::Jnz((r, v)) => {
                if let Some(elem) = registers.get_mut(*r) {
                    if *elem != 0 {
                        position += *v;
                    } else {
                        position += 1;
                    }
                }
            }
            Inst::JnzOther((elem, v)) => {
                if *elem != 0 {
                    position += *v;
                } else {
                    position += 1;
                }
            }
        }
    }
    println!("{}", registers[0])
}

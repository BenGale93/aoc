use aoc_utils::{Cli, puzzle_input_lines};

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl Register {
    fn from_str(input: &str) -> Self {
        match input {
            "a" => Self::A,
            "b" => Self::B,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Instructions {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie((Register, isize)),
    Jio((Register, isize)),
}

impl Instructions {
    fn from_str(input: String) -> Self {
        let elements: Vec<_> = input.split(" ").collect();

        match *elements.first().unwrap() {
            "hlf" => Self::Hlf(Register::from_str(elements.last().unwrap())),
            "tpl" => Self::Tpl(Register::from_str(elements.last().unwrap())),
            "inc" => Self::Inc(Register::from_str(elements.last().unwrap())),
            "jmp" => Self::Jmp(elements.last().unwrap().parse().unwrap()),
            "jio" => Self::Jio((
                Register::from_str(elements[1].strip_suffix(',').unwrap()),
                elements.last().unwrap().parse().unwrap(),
            )),
            "jie" => Self::Jie((
                Register::from_str(elements[1].strip_suffix(',').unwrap()),
                elements.last().unwrap().parse().unwrap(),
            )),
            _ => panic!(),
        }
    }
}

fn main() {
    let instructions: Vec<_> = puzzle_input_lines("input")
        .map(Result::unwrap)
        .map(Instructions::from_str)
        .collect();

    let mut registers: (usize, usize) = if Cli::parse_args().part_two {
        (1, 0)
    } else {
        (0, 0)
    };
    let mut pointer: isize = 0;

    loop {
        if pointer < 0 {
            break;
        }
        let instruction = instructions.get(pointer as usize);
        let Some(inst) = instruction else {
            break;
        };

        pointer += 1;

        match inst {
            Instructions::Hlf(register) => match register {
                Register::A => registers.0 /= 2,
                Register::B => registers.1 /= 2,
            },
            Instructions::Tpl(register) => match register {
                Register::A => registers.0 *= 3,
                Register::B => registers.1 *= 3,
            },
            Instructions::Inc(register) => match register {
                Register::A => registers.0 += 1,
                Register::B => registers.1 += 1,
            },
            Instructions::Jmp(offset) => pointer += offset - 1,
            Instructions::Jie((register, offset)) => match register {
                Register::A => {
                    if registers.0 % 2 == 0 {
                        pointer += offset - 1;
                    }
                }
                Register::B => {
                    if registers.1 % 2 == 0 {
                        pointer += offset - 1;
                    }
                }
            },
            Instructions::Jio((register, offset)) => match register {
                Register::A => {
                    if registers.0 == 1 {
                        pointer += offset - 1;
                    }
                }
                Register::B => {
                    if registers.1 == 1 {
                        pointer += offset - 1;
                    }
                }
            },
        };
        println!("a: {}, b: {}", registers.0, registers.1);
    }
}

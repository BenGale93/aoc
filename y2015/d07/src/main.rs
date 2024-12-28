use std::collections::HashMap;

use aoc_utils::puzzle_input_lines;

#[derive(Debug, Clone)]
enum Instruction {
    Const(u16),
    And((String, String)),
    NumAnd((u16, String)),
    Or((String, String)),
    Lshift((String, u16)),
    Rshift((String, u16)),
    Not(String),
    Pass(String),
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let potential_const: Result<u16, _> = instruction.parse();
        if let Ok(c) = potential_const {
            return Self::Const(c);
        };

        if instruction.contains("AND") {
            let parts: Vec<_> = instruction
                .split("AND")
                .map(|s| s.trim_end().trim_start().to_string())
                .collect();
            let num: Result<u16, _> = parts.first().unwrap().parse();
            if let Ok(n) = num {
                return Self::NumAnd((n, parts.last().unwrap().to_string()));
            };
            return Self::And((
                parts.first().unwrap().to_string(),
                parts.last().unwrap().to_string(),
            ));
        };

        if instruction.contains("OR") {
            let parts: Vec<_> = instruction
                .split("OR")
                .map(|s| s.trim_end().trim_start().to_string())
                .collect();
            return Self::Or((
                parts.first().unwrap().to_string(),
                parts.last().unwrap().to_string(),
            ));
        };

        if instruction.contains("LSHIFT") {
            let parts: Vec<_> = instruction
                .split("LSHIFT")
                .map(|s| s.trim_end().trim_start().to_string())
                .collect();
            return Self::Lshift((
                parts.first().unwrap().to_string(),
                parts.last().unwrap().parse().unwrap(),
            ));
        };

        if instruction.contains("RSHIFT") {
            let parts: Vec<_> = instruction
                .split("RSHIFT")
                .map(|s| s.trim_end().trim_start().to_string())
                .collect();
            return Self::Rshift((
                parts.first().unwrap().to_string(),
                parts.last().unwrap().parse().unwrap(),
            ));
        };

        if instruction.contains("NOT") {
            let parts: Vec<_> = instruction
                .split("NOT")
                .map(|s| s.trim_end().trim_start().to_string())
                .collect();
            return Self::Not(parts.last().unwrap().to_string());
        }
        Self::Pass(instruction.to_string())
    }

    fn compute(&self, inputs: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Instruction::Const(c) => Some(*c),
            Instruction::And((l, r)) => {
                let (Some(left), Some(right)) = (inputs.get(l), inputs.get(r)) else {
                    return None;
                };
                Some(left & right)
            }
            Instruction::NumAnd((n, right)) => {
                let value = inputs.get(right)?;
                Some(n & value)
            }
            Instruction::Or((l, r)) => {
                let (Some(left), Some(right)) = (inputs.get(l), inputs.get(r)) else {
                    return None;
                };
                Some(left | right)
            }
            Instruction::Lshift((v, a)) => {
                let value = inputs.get(v)?;
                Some(value << a)
            }
            Instruction::Rshift((v, a)) => {
                let value = inputs.get(v)?;
                Some(value >> a)
            }
            Instruction::Not(v) => {
                let value = inputs.get(v)?;
                Some(!value)
            }
            Instruction::Pass(v) => {
                let value = inputs.get(v)?;
                Some(*value)
            }
        }
    }
}

fn main() {
    let mut instructions = vec![];
    for line in puzzle_input_lines("input") {
        let line = line.unwrap();
        let parts: Vec<_> = line
            .split("->")
            .map(|s| s.trim_end().trim_start().to_string())
            .collect();
        let instruction = Instruction::new(parts.first().unwrap());
        let destination = parts.last().unwrap().to_string();
        instructions.push((instruction, destination));
    }
    let mut output_values = HashMap::new();
    let a = loop {
        for (instruction, destination) in &instructions {
            let output = instruction.compute(&output_values);
            if let Some(o) = output {
                output_values.insert(destination.clone(), o);
            }
        }
        if let Some(v) = output_values.get("a") {
            break v;
        }
    };
    println!("Part one: {}", a);

    let new_instructions: Vec<_> = instructions
        .iter()
        .map(|(i, d)| {
            if d == "b" {
                (Instruction::Const(*a), d)
            } else {
                (i.clone(), d)
            }
        })
        .collect();

    let mut output_values = HashMap::new();
    let a = loop {
        for (instruction, destination) in &new_instructions {
            let output = instruction.compute(&output_values);
            if let Some(o) = output {
                output_values.insert(destination.to_string(), o);
            }
        }
        if let Some(v) = output_values.get("a") {
            break v;
        }
    };
    println!("Part two: {}", a);
}

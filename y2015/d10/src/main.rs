use aoc_utils::Cli;

fn main() {
    let mut input = "1113222113".to_string();

    let iterations = if Cli::parse_args().part_two { 50 } else { 40 };

    for _ in 0..iterations {
        input = look_and_say(&input);
    }

    println!("{}", input.len());
}

fn look_and_say(input: &str) -> String {
    let mut counts = vec![];
    let mut prev = 'x';
    for d in input.chars() {
        if prev != d {
            prev = d;
            counts.push((d, 1));
        } else {
            let len = counts.len() - 1;
            counts.get_mut(len).unwrap().1 += 1;
        }
    }

    let mut output = "".to_string();

    for (v, c) in counts {
        let count = c.to_string();
        output = output + &count;
        output.push(v);
    }
    output
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn basic_count(#[case] input: &str, #[case] output: &str) {
        assert_eq!(look_and_say(input), output);
    }
}

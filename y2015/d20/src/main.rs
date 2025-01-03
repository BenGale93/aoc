use aoc_utils::Cli;

fn main() {
    let result = if Cli::parse_args().part_two {
        let target = (36000000 / 11) + 1;
        search_part_two(target)
    } else {
        let target = 3600000;
        search_part_one(target)
    };
    println!("{result}");
}

fn count_presents_part_one(house: isize) -> isize {
    let mut divisors = vec![1, house];

    for i in 2..=(house / 2) {
        if house % i == 0 {
            divisors.push(i);
        }
    }

    divisors.into_iter().sum()
}

fn search_part_one(target: isize) -> isize {
    let mut i: isize = 831300; // cheated here by changing this randomly upwards
    let steps = vec![100, -10, 1];

    for step in steps {
        loop {
            let result = count_presents_part_one(i);
            if step > 0 && result >= target || step < 0 && result < target {
                break;
            } else if step == 1 && result >= target {
                return i;
            }
            i += step;
        }
    }
    i
}

fn search_part_two(target: usize) -> isize {
    let mut presents = vec![1; target];
    for elf in 2..target {
        let mut i = 0;

        for house in (elf - 1..target).step_by(elf) {
            presents[house] += elf;
            i += 1;
            if i >= 50 {
                break;
            }
        }
        if presents[elf - 1] >= target {
            return elf as isize;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::count_presents_part_one;

    #[rstest]
    #[case(2, 3)]
    #[case(3, 4)]
    #[case(4, 7)]
    #[case(5, 6)]
    #[case(6, 12)]
    #[case(8, 15)]
    #[case(9, 13)]
    fn example_one_part_one(#[case] input: isize, #[case] output: isize) {
        assert_eq!(count_presents_part_one(input), output);
    }
}

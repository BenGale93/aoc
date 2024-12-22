use aoc_utils::{get_entire_puzzle, Cli};

fn main() {
    let input = get_entire_puzzle("input");
    let key = input.first().unwrap();
    let number = if Cli::parse_args().part_two { 6 } else { 5 };
    let result = hasher(key, number);

    println!("{result}")
}

fn hasher(key: &str, number: usize) -> u32 {
    for i in 1..u32::MAX {
        let test_key = format!("{key}{i}");
        let digest = md5::compute(test_key);
        let hash = format!("{:x}", digest);
        let zeros = hash.chars().take(number).all(|c| c == '0');
        if zeros {
            return i;
        }
    }
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        assert_eq!(hasher("abcdef", 5), 609043);
    }

    #[test]
    fn part_one_example2() {
        assert_eq!(hasher("pqrstuv", 5), 1048970);
    }
}

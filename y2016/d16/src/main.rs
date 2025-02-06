use aoc_utils::Cli;
use itertools::Itertools;

fn expand(a: &str) -> String {
    let b: String = a
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unimplemented!(),
        })
        .collect();

    format!("{a}0{b}")
}

fn expand_to_size(input: &str, length: usize) -> String {
    let mut a = expand(input);
    loop {
        if a.len() >= length {
            break;
        }
        a = expand(&a);
    }
    a.chars().take(length).collect()
}

fn checksum_inner(input: &str) -> String {
    input
        .chars()
        .tuples()
        .map(|(a, b)| if a == b { '1' } else { '0' })
        .collect()
}

fn checksum(input: &str) -> String {
    let mut a = checksum_inner(input);
    loop {
        if a.len() % 2 == 1 {
            break;
        }
        a = checksum_inner(&a);
    }
    a
}

fn main() {
    let input = "10001001100000001";
    let length = if Cli::parse_args().part_two {
        35651584
    } else {
        272
    };
    let expanded = expand_to_size(input, length);
    let checksum = checksum(&expanded);

    println!("{checksum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        assert_eq!(expand("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn test_expand_to_size() {
        assert_eq!(expand_to_size("10000", 20), "10000011110010000111");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("110010110100"), "100");
    }
}

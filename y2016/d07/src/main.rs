use aoc_utils::{Cli, puzzle_input_lines};
use itertools::Itertools;

fn main() {
    let support = if Cli::parse_args().part_two {
        supports_ssl
    } else {
        supports_tls
    };
    let total = puzzle_input_lines("input")
        .map(Result::unwrap)
        .map(|ip| support(&ip))
        .filter(|b| *b)
        .count();

    println!("{total}");
}

fn valid_split(c: char) -> bool {
    c == '[' || c == ']'
}

fn supports_tls(ip: &str) -> bool {
    let sub_ips: Vec<_> = ip.split(valid_split).collect();

    let mut hypernet = false;
    let mut valid = false;
    for sub_ip in sub_ips {
        let supports_tls = has_abba(sub_ip);

        if supports_tls && hypernet {
            return false;
        }

        if supports_tls && !hypernet {
            valid = true
        }
        hypernet = !hypernet;
    }
    valid
}

fn has_abba(input: &str) -> bool {
    for (a, b, c, d) in input.chars().tuple_windows() {
        if a == d && b == c && a != b {
            return true;
        }
    }
    false
}

fn supports_ssl(ip: &str) -> bool {
    let sub_ips: Vec<_> = ip.split(valid_split).collect();
    let mut supernets = vec![];
    let mut hypernets = vec![];

    let mut supernet = true;
    for sub_ip in sub_ips {
        if supernet {
            supernets.push(sub_ip);
        } else {
            hypernets.push(sub_ip);
        }
        supernet = !supernet;
    }

    let mut babs = vec![];
    for sub_ip in supernets {
        babs.extend(valid_babs(sub_ip));
    }

    for bab in babs {
        for hypernet in &hypernets {
            if hypernet.contains(&bab) {
                return true;
            }
        }
    }
    false
}

fn valid_babs(input: &str) -> Vec<String> {
    let mut abas = vec![];
    for (a, b, c) in input.chars().tuple_windows() {
        if a == c && a != b {
            abas.push([b, a, b].iter().collect());
        }
    }
    abas
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("abba[mnop]qrst", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn part_one(#[case] input: &str, #[case] output: bool) {
        assert_eq!(supports_tls(input), output);
    }
}

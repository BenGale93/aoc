use aoc_utils::Cli;

fn get_password_part_one(door: &str) -> String {
    let mut password: Vec<char> = vec![];
    let mut index = 0;
    while password.len() < 8 {
        let candidate = format!("{door}{index}");
        let digest = md5::compute(candidate);
        let hash = format!("{:x}", digest);
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
        }
        index += 1;
    }

    password.iter().collect()
}

fn get_password_part_two(door: &str) -> String {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut index = 0;
    loop {
        let candidate = format!("{door}{index}");
        let digest = md5::compute(candidate);
        let hash = format!("{:x}", digest);
        if hash.starts_with("00000") {
            let mut chars = hash.chars();
            let maybe_pos = chars.nth(5).unwrap();
            if let Some(pos) = maybe_pos.to_digit(10) {
                if let Some(value) = password.get_mut(pos as usize) {
                    if value.is_none() {
                        *value = Some(chars.next().unwrap());
                    }
                }
            };
        }
        if password.iter().all(|v| v.is_some()) {
            break;
        }
        index += 1;
    }
    password.iter().map(|c| c.unwrap()).collect()
}

fn main() {
    let input = "ugkcyxxp";

    let password = if Cli::parse_args().part_two {
        get_password_part_two(input)
    } else {
        get_password_part_one(input)
    };

    println!("{password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_one() {
        assert_eq!(get_password_part_one("abc"), "18f47a30")
    }

    #[test]
    fn part_two_example_one() {
        assert_eq!(get_password_part_two("abc"), "05ace8e3")
    }
}

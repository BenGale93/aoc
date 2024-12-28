use aoc_utils::puzzle_input_lines;

fn main() {
    let lines = puzzle_input_lines("input");
    let re = regex::Regex::new(r"\\x[0-9a-f][0-9a-f]").unwrap();

    let mut code_chars = 0;
    let mut value_chars = 0;
    let mut encoded_chars = 0;
    for line in lines {
        let line = line.unwrap();
        code_chars += line.len();

        let value = line
            .strip_suffix('"')
            .unwrap()
            .strip_prefix('"')
            .unwrap()
            .replace(r#"\\"#, r#"\"#)
            .replace(r#"\""#, r#"""#);
        let sub = re.replace_all(&value, "Z");
        value_chars += sub.len();

        let encoded = line.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#);
        encoded_chars += encoded.len() + 2;
    }

    println!("Part one: {}", code_chars - value_chars);
    println!("Part two: {}", encoded_chars - code_chars);
}

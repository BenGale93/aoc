use aoc_utils::Cli;

fn main() {
    let mut raw_remainders: Vec<i64> = vec![1, 0, 2, 0, 0, 5];
    let mut numbers: Vec<i64> = vec![17, 7, 19, 5, 3, 13];

    if Cli::parse_args().part_two {
        raw_remainders.push(0);
        numbers.push(11);
    }

    let mut remainders = vec![];

    for (i, remainder) in raw_remainders.iter().enumerate() {
        let num = numbers[i];
        remainders.push(num - (remainder + i as i64 + 1) % num);
    }

    println!("{}", chinese_remainder_theorem(&numbers, &remainders));
}

fn invert(a: i64, m: i64) -> i64 {
    let mut u1 = 1;
    let mut u2 = 0;
    let mut u3 = a;

    let mut v1 = 0;
    let mut v2 = 1;
    let mut v3 = m;

    while v3 != 0 {
        let q = u3 / v3;
        let x1 = u1 - q * v1;
        let x2 = u2 - q * v2;
        let x3 = u3 - q * v3;

        u1 = v1;
        u2 = v2;
        u3 = v3;

        v1 = x1;
        v2 = x2;
        v3 = x3;
    }

    u1 % m
}

fn chinese_remainder_theorem(numbers: &[i64], remainders: &[i64]) -> i64 {
    let product = numbers.iter().product::<i64>();
    let mut result = 0;

    for (i, remainder) in remainders.iter().enumerate() {
        let pp = product / numbers[i];
        result += *remainder * invert(pp, numbers[i]) * pp
    }

    result.rem_euclid(product)
}

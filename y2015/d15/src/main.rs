use aoc_utils::{Cli, FromRegex, parse_with_regex};

#[derive(Debug)]
struct Stats {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl FromRegex for Stats {
    fn from_regex(line: &str, re: &regex::Regex) -> Self {
        let capture = re.captures(line).unwrap();
        Self {
            capacity: capture.get(1).unwrap().as_str().parse().unwrap(),
            durability: capture.get(2).unwrap().as_str().parse().unwrap(),
            flavor: capture.get(3).unwrap().as_str().parse().unwrap(),
            texture: capture.get(4).unwrap().as_str().parse().unwrap(),
            calories: capture.get(5).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn main() {
    let re = regex::Regex::new(
        r"capacity (-?\d*), durability (-?\d*), flavor (-?\d*), texture (-?\d*), calories (-?\d*)",
    )
    .unwrap();
    let stats: Vec<Stats> = parse_with_regex("input", re);

    let score = if Cli::parse_args().part_two {
        brute_force_calories(&stats)
    } else {
        gradient_ascent(&stats)
    };

    println!("{}", score)
}

fn gradient_ascent(stats: &[Stats]) -> isize {
    let ingredient_count = stats.len();
    let starting_counts = (100 / ingredient_count) as isize;

    let mut counts = vec![starting_counts; ingredient_count];
    let mut current_score = compute_score(&counts, stats);
    loop {
        let mut score_improvement = vec![];
        for i in 0..ingredient_count {
            for j in 0..ingredient_count {
                if i == j {
                    continue;
                }
                let mut count_copy = counts.clone();
                count_copy[i] -= 1;
                count_copy[j] += 1;
                let new_score = compute_score(&count_copy, stats);
                score_improvement.push((new_score, count_copy));
            }
        }
        let best_score = score_improvement.iter().max_by_key(|x| x.0).unwrap();
        if best_score.0 > current_score {
            current_score = best_score.0;
            counts = best_score.1.clone();
        } else {
            return current_score;
        }
    }
}

fn brute_force_calories(stats: &[Stats]) -> isize {
    let mut valid_combos = vec![];

    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;
                let count = vec![i, j, k, l];
                let calories = compute_calories(&count, stats);
                if calories == 500 {
                    valid_combos.push(count.clone())
                }
            }
        }
    }

    valid_combos
        .iter()
        .map(|c| compute_score(c, stats))
        .max()
        .unwrap()
}

fn compute_score(counts: &[isize], stats: &[Stats]) -> isize {
    assert_eq!(counts.len(), stats.len());

    let capacity = stats
        .iter()
        .map(|s| s.capacity)
        .zip(counts.iter())
        .map(|(cap, count)| cap * *count)
        .sum::<isize>();
    let durability = stats
        .iter()
        .map(|s| s.durability)
        .zip(counts.iter())
        .map(|(cap, count)| cap * *count)
        .sum::<isize>();
    let flavor = stats
        .iter()
        .map(|s| s.flavor)
        .zip(counts.iter())
        .map(|(cap, count)| cap * *count)
        .sum::<isize>();
    let texture = stats
        .iter()
        .map(|s| s.texture)
        .zip(counts.iter())
        .map(|(cap, count)| cap * *count)
        .sum::<isize>();

    floor(capacity) * floor(durability) * floor(flavor) * floor(texture)
}

fn floor(value: isize) -> isize {
    if value < 0 {
        return 0;
    }
    value
}

fn compute_calories(counts: &[isize], stats: &[Stats]) -> isize {
    stats
        .iter()
        .zip(counts.iter())
        .map(|(s, c)| s.calories * c)
        .sum()
}

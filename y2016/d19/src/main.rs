use std::collections::VecDeque;

fn part_one(n: usize) -> usize {
    let mut circle: VecDeque<usize> = (1..n + 1).collect();
    let mut result = 0;
    while let Some(e) = circle.pop_front() {
        if circle.is_empty() {
            result = e;
        }
        circle.push_back(e);
        circle.pop_front();
    }
    result
}

fn part_two(n: usize) -> usize {
    let mut left: VecDeque<_> = (1..=(n + 1) / 2).collect();
    let mut right: VecDeque<_> = ((n + 1) / 2 + 1..n + 1).rev().collect();
    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }
    left.pop_front().unwrap()
}

fn main() {
    let input = 3014387;
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

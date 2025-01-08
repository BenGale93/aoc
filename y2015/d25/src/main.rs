fn main() {
    let final_row = 2978;
    let final_column = 3083;

    let mut value: usize = 20151125;

    let mut row = 2;
    let mut max_row = 2;
    let mut column = 1;
    loop {
        value = compute(value);

        if row == final_row && column == final_column {
            break;
        }

        if row == 1 {
            row = max_row + 1;
            max_row = row;
            column = 1;
        } else {
            row -= 1;
            column += 1;
        }
    }

    println!("{value}");
}

fn compute(value: usize) -> usize {
    (value * 252533) % 33554393
}

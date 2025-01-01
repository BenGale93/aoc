use std::fs::read_to_string;

use aoc_utils::{Cli, Coord, FromChar, out_of_bounds, puzzle_matrix};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Light {
    On,
    Off,
}

impl FromChar for Light {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::On,
            '.' => Self::Off,
            _ => panic!("Unrecognised pattern."),
        }
    }
}

impl Light {
    const fn switch(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }

    const fn value(self) -> usize {
        match self {
            Self::On => 1,
            Self::Off => 0,
        }
    }
}

fn main() {
    let mut grid = puzzle_matrix::<Light>(&read_to_string("input").unwrap());
    let size = grid.len() as isize;

    let update = if Cli::parse_args().part_two {
        grid = edges_on(grid, size);
        update_part_two
    } else {
        update_part_one
    };

    for _ in 0..100 {
        grid = update(&grid, size);
    }

    let total: usize = grid
        .iter()
        .map(|r| r.iter().map(|l| l.value()).sum::<usize>())
        .sum();

    println!("{total}");
}

fn update_part_one(grid: &[Vec<Light>], size: isize) -> Vec<Vec<Light>> {
    let mut new_grid = grid.to_vec();

    for (i, row) in grid.iter().enumerate() {
        for (j, light) in row.iter().enumerate() {
            let coords = (i as isize, j as isize);
            let neighbors = count_neighbours(grid, &coords, size);
            let new_light = match light {
                Light::On if !(2..=3).contains(&neighbors) => light.switch(),
                Light::Off if neighbors == 3 => light.switch(),
                _ => *light,
            };
            new_grid[i][j] = new_light;
        }
    }

    new_grid.to_vec()
}

fn update_part_two(grid: &[Vec<Light>], size: isize) -> Vec<Vec<Light>> {
    let new_grid = update_part_one(grid, size);
    edges_on(new_grid, size)
}

fn edges_on(mut grid: Vec<Vec<Light>>, size: isize) -> Vec<Vec<Light>> {
    let size = (size - 1) as usize;
    grid[0][0] = Light::On;
    grid[0][size] = Light::On;
    grid[size][0] = Light::On;
    grid[size][size] = Light::On;

    grid
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbours(grid: &[Vec<Light>], coords: &Coord, size: isize) -> usize {
    let mut on_neighbours = 0;
    for neighbour_offset in NEIGHBOURS {
        let neighbour = (coords.0 + neighbour_offset.0, coords.1 + neighbour_offset.1);
        if out_of_bounds(&neighbour, size) {
            continue;
        }
        if let Light::On = grid[neighbour.0 as usize][neighbour.1 as usize] {
            on_neighbours += 1
        };
    }

    on_neighbours
}

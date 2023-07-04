use itertools::Itertools;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let puzzle = get_puzzle(filename);
    let result_part1 = part_1(&puzzle);
    println!("Part 1: {}", result_part1);
    let result_part2 = part_2(&puzzle);
    println!("Part 2: {}", result_part2);
}

fn get_puzzle(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}

fn process_puzzle(puzzle: &str) -> Vec<Vec<u32>> {
    return puzzle
        .lines()
        .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect();
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let col = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (left, right) = row.split_at(x);
    let (up, down) = col.split_at(y);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    return [up, down, left, right];
}

fn part_1(puzzle: &str) -> usize {
    let grid = process_puzzle(puzzle);
    let len = grid.len();

    return (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = grid[y][x];

            directions(&grid, x, y)
                .iter()
                .map(|dir| dir.iter().all(|h| h < &height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (len - 1) * 4;
}

fn part_2(puzzle: &str) -> usize {
    let grid = process_puzzle(puzzle);
    let len = grid.len();

    return (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = grid[y][x];

            directions(&grid, x, y)
                .iter()
                .map(|dir| {
                    dir.iter()
                        .position(|h| h >= &height)
                        .map(|pos| pos + 1)
                        .unwrap_or_else(|| dir.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap();
}

use itertools::Itertools;
use std::collections::HashSet;
use std::{env, fs};

fn get_puzzle_input(path: &String) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn part_1(puzzle: Vec<String>) -> usize {
    let start = Coordinate { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    let mut visited = HashSet::new();
    visited.insert(start);

    for line in puzzle {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<isize>().unwrap();
        for _ in 0..distance {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("Unknown direction"),
            }
            let diff = Coordinate {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            let not_adjacent = diff.x.abs() > 1 || diff.y.abs() > 1;
            if not_adjacent {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                visited.insert(tail);
            }
        }
    }
    return visited.len();
}

fn part_2(puzzle: Vec<String>) -> usize {
    let start = Coordinate { x: 0, y: 0 };
    let mut rope = vec![start; 10];
    let mut visited = HashSet::new();
    visited.insert(start);

    for line in puzzle {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance: usize = distance.parse().unwrap();
        for _ in 0..distance {
            match direction {
                "U" => rope[0].y -= 1,
                "D" => rope[0].y += 1,
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                _ => panic!("Unknown direction"),
            }
            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                let diff = Coordinate {
                    x: rope[head_idx].x - rope[tail_idx].x,
                    y: rope[head_idx].y - rope[tail_idx].y,
                };
                let not_adjacent = diff.x.abs() > 1 || diff.y.abs() > 1;
                if not_adjacent {
                    rope[tail_idx].x += diff.x.signum();
                    rope[tail_idx].y += diff.y.signum();
                    if tail_idx == rope.len() - 1 {
                        visited.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }
    return visited.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let puzzle = get_puzzle_input(filename);
    //println!("Part 1: {}", part_1(puzzle));
    println!("Part 2: {}", part_2(puzzle));
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

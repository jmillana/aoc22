use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let part_1_total = part_1(&contents);
    let part_2_total = part_2(&contents);
    println!("Part 1: {}", part_1_total);
    println!("Part 2: {}", part_2_total);
}

fn filter_items(mut unique: Vec<char>, items: &Vec<char>) -> Vec<char> {
    let mut new_unique: Vec<char> = Vec::new();
    if unique.len() == 0 {
        unique = items.clone();
    }
    for item in unique {
        if items.contains(&item) {
            if !new_unique.contains(&item) {
                new_unique.push(item);
            }
        }
    }
    return new_unique;
}

fn part_2(content: &String) -> u32 {
    let mut total = 0;
    let mut elf = 0;
    let mut unique: Vec<char> = Vec::new();
    let mut group = 0;
    for line in content.lines() {
        unique = filter_items(unique, &line.chars().collect());
        if elf % 3 == 2 {
            group += 1;
            println!("Group {}: Item: {:?}", group, unique);
            total += letter_to_number(unique[0]);
            unique = Vec::new();
        }
        elf += 1;
    }
    return total;
}

fn part_1(content: &String) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        match process_lines(line) {
            Some(c) => {
                println!("Item: {}", c);
                total += letter_to_number(c);
            }
            None => println!("Not found"),
        }
    }
    return total;
}

fn letter_to_number(c: char) -> u32 {
    match c {
        'a'..='z' => return c as u32 - 'a' as u32 + 1,
        'A'..='Z' => return c as u32 - 'A' as u32 + 27,
        _ => return 0,
    }
}

fn process_lines(line: &str) -> Option<char> {
    let size = line.len();
    let part_1_chars = line[0..(size / 2)].chars();
    let part_2 = line[(size / 2)..].to_string();
    for c1 in part_1_chars {
        if let Some(_) = part_2.find(c1) {
            return Some(c1);
        }
    }
    return None;
}

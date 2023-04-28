use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let snaks_file = "target/snaks.txt";
    let file = File::open(snaks_file).expect("File ${path} not found");
    let reader = BufReader::new(file);
    parse_calories3(reader.lines());
}

pub struct Inventory {
    snaks: Vec<i32>,
    total_calories: i32,
}

impl Inventory {
    pub fn new() -> Self {
        return Inventory {
            snaks: Vec::new(),
            total_calories: 0,
        };
    }

    pub fn add_calories(&mut self, calories: i32) {
        self.snaks.push(calories);
        self.total_calories += calories;
    }
}

// Minimum stars 5
pub fn parse_calories(data: Lines<BufReader<File>>) -> Vec<Inventory> {
    let mut inventory_list = Vec::new();
    let mut inventory = Inventory::new();
    let mut calories: i32;
    for line in data {
        let line_data = line.unwrap();

        if line_data == "" {
            inventory = Inventory::new();
            inventory_list.push(inventory);
        } else {
            calories = line_data.parse().unwrap();
            //inventory.add_calories(calories);
        }
    }
    return inventory_list;
}

pub fn parse_calories2(data: Lines<BufReader<File>>) -> Vec<i32> {
    let mut max_elf = 0;
    let mut max_calories = 0;
    let mut current_elf = 0;
    let mut current_calories = 0;

    for line in data {
        let line_data = line.unwrap();

        if line_data == "" {
            if current_calories > max_calories {
                max_elf = current_elf;
                max_calories = current_calories;
            }
            current_elf += 1;
            current_calories = 0;
        } else {
            let calories: i32 = line_data.parse().unwrap();
            current_calories += calories;
        }
    }
    println!("Max elf {max_elf}, calories: {max_calories}");
    return vec![max_elf, max_calories];
}

pub fn parse_calories3(data: Lines<BufReader<File>>) {
    let mut top3_elf: Vec<i32> = vec![0, 0, 0];
    let mut top3_calories: Vec<i32> = vec![0, 0, 0];
    let mut current_elf = 0;
    let mut current_calories = 0;

    for line in data {
        let line_data = line.unwrap();

        if line_data == "" {
            let mut min_value = top3_calories[0];
            let mut min_idx = 0;
            // Get the minimum value and index of the top 3
            for (idx, calories) in top3_calories.iter().enumerate() {
                if calories.clone() < min_value {
                    min_idx = idx;
                    min_value = calories.clone();
                }
            }
            if current_calories > min_value {
                top3_calories[min_idx] = current_calories;
                top3_elf[min_idx] = current_elf;
            }

            current_elf += 1;
            current_calories = 0;
        } else {
            let calories: i32 = line_data.parse().unwrap();
            current_calories += calories;
        }
    }
    let total_calories: i32 = top3_calories.iter().sum();
    println!("Top 3 calories: {}", total_calories);
}

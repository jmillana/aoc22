use std::{env, fs};

const OPTIONS: [&str; 3] = ["Rock", "Paper", "Scisors"];
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut score = 0;
    for line in contents.lines() {
        let mut parts = line.split(" ");
        let oponent = character_to_number(parts.nth(0).unwrap());
        let player = character_to_number(parts.nth(0).unwrap());
        score += play(player, oponent);
    }
    println!("Final score: {}", score);
}

fn play(player: usize, oponent: usize) -> usize {
    // Add 3 to avoid negative numbers
    // Add 1 to shift the result so 0 is lose, 1 is draw and 2 is win
    let result = (3 + 1 + player - oponent) % 3;
    print!("{} vs {}: ", OPTIONS[player], OPTIONS[oponent]);
    match result {
        0 => println!("Lose"),
        1 => println!("Draw"),
        2 => println!("Win"),
        _ => panic!("Invalid result"),
    };
    println!("{}", 1 + player + (result * 3));
    return 1 + player + (result * 3);
}

fn character_to_number(character: &str) -> usize {
    match character {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("Invalid character"),
    }
}

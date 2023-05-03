use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in contents.lines() {
        println!("{}", transmision_start(line, 14));
    }
}

fn transmision_start(line: &str, seq_size: usize) -> usize {
    let mut start = 0;
    let mut queue: VecDeque<char> = VecDeque::new();
    for character in line.chars() {
        while is_duplicate(&character, &queue) {
            queue.pop_front();
        }

        queue.push_back(character);
        start += 1;
        if queue.len() == seq_size {
            break;
        }
    }
    return start;
}

fn is_duplicate(character: &char, queue: &VecDeque<char>) -> bool {
    for item in queue {
        if item == character {
            return true;
        }
    }
    return false;
}

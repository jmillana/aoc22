use regex::Regex;
use std::collections::VecDeque;
use std::{env, fmt, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut cargo = Cargo::load_cargo(&contents);
    println!("{}", cargo);
    let mut to_skip = cargo.size() + 2;
    for line in contents.lines() {
        if to_skip > 0 {
            to_skip -= 1;
            continue;
        }
        let action = Action::build_from_string(line);
        // cargo.move_package(action);
        cargo.move_stacked(action);
        println!("{}", cargo);
    }
    println!("Results: \n{}", cargo);
    println!("Solution: {}", cargo.result());
}

struct Cargo {
    stacks: Vec<VecDeque<char>>,
}

struct Action {
    n: u32,
    from: usize,
    to: usize,
}

impl Action {
    fn build(n: u32, from: usize, to: usize) -> Action {
        return Action { n, from, to };
    }

    pub fn build_from_string(line: &str) -> Self {
        let (n, from, to) = Action::parse_actions(line);
        return Action::build(n, from, to);
    }

    fn parse_actions(line: &str) -> (u32, usize, usize) {
        let re = Regex::new(r"(\d+).*(\d+).*(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let n = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        return (n, from - 1, to - 1);
    }
}

impl Cargo {
    fn load_cargo(data: &String) -> Self {
        let mut counter;
        let first_line = data.lines().into_iter().next();
        let line_size = first_line.unwrap().chars().count();
        let cargo_capacity = (line_size + 1) / 4;
        let mut cargo: Vec<VecDeque<char>> = Vec::with_capacity(cargo_capacity);
        for _ in 0..cargo_capacity {
            cargo.push(VecDeque::new());
        }
        'cargo: for line in data.lines() {
            counter = 0;
            let mut stack_idx: usize = 0;
            for package in line.chars() {
                if counter % 4 == 1 {
                    if package == '1' {
                        break 'cargo;
                    }
                    if package != ' ' {
                        cargo[stack_idx].push_front(package.clone());
                    }
                    stack_idx = (stack_idx + 1) % cargo_capacity;
                }
                counter += 1;
            }
        }
        return Cargo { stacks: cargo };
    }

    fn size(&self) -> u32 {
        return self.stacks.iter().map(|stack| stack.len()).max().unwrap() as u32;
    }

    fn move_package(&mut self, action: Action) {
        for _ in 0..action.n {
            let package = self.stacks[action.from].pop_back().unwrap();
            self.stacks[action.to].push_back(package);
        }
    }

    fn move_stacked(&mut self, action: Action) {
        let mut helper_stack = Vec::new();
        for _ in 0..action.n {
            let package = self.stacks[action.from].pop_back().unwrap();
            helper_stack.push(package);
        }
        for _ in 0..action.n {
            let package = helper_stack.pop().unwrap();
            self.stacks[action.to].push_back(package);
        }
    }

    fn result(&self) -> String {
        let mut result = String::new();
        for stack in &self.stacks {
            result.push(*stack.back().unwrap());
        }
        return result;
    }
}

impl fmt::Display for Cargo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let max_size = self.size();
        for level in (0..max_size).rev() {
            for stack in &self.stacks {
                if stack.len() > level as usize {
                    result.push_str(&format!("[{}] ", stack[level as usize]));
                } else {
                    result.push_str("    ");
                }
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

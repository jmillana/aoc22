use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut root = Node::new(); //HashMap<&str, Node> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Unable to read the file {filename}");
    let mut current_node = &mut root;
    // let mut current_dir = Directory::build("".to_string(), None);
    for line in contents.lines() {
        let action = parse_line(line);
        println!("{:?}", action);
        match action {
            Line::Command(cmd) => match cmd {
                Command::CD(path) => match path.as_str() {
                    ".." => {
                        stack.pop();
                        current_node = &mut root;
                        for key in &stack {
                            println!("key: {}", key);
                            current_node = current_node.children.get_mut(key.as_str()).unwrap();
                        }
                    }
                    _ => {
                        current_node.children.entry(path.clone()).or_default();
                        stack.push(path.clone());
                        current_node = current_node.children.get_mut(path.as_str()).unwrap();
                    }
                },
                Command::LS => {}
            },
            Line::Response(response) => match response {
                Response::Directory(path) => {
                    current_node.children.entry(path.clone()).or_default();
                }
                Response::File(path, size) => {
                    current_node.children.entry(path.clone()).or_default().size = size;
                }
            },
        }
    }
    root.update_size();
    println!("{root:#?}");
    let dirs = get_dirs_with_size_under(&root, 100000);
    let mut total_size = 0;
    for (dir, size) in dirs {
        total_size += size;
        println!("{}: {}", dir, size);
    }
    println!("Total size: {}", total_size);
    let system_space = 70000000;
    let required_space = 30000000;
    let used_space = root.size;
    let free_space = system_space - used_space;
    let missing_space = required_space - free_space;
    println!("Missing space: {}", missing_space);
    let candidates = get_dirs_with_size_over(&root, missing_space);
    let mut smallest = candidates[0].1;
    let mut smallest_dir = candidates[0].0.clone();
    for (dir, size) in candidates {
        println!("{}: {}", &dir, size);
        if size < smallest {
            smallest = size;
            smallest_dir = dir;
        }
    }
    println!("Smallest: {}, size: {}", smallest_dir, smallest);
}

fn parse_cmd(input: &str) -> Command {
    match input[..2].to_string().as_str() {
        "cd" => Command::CD(input[3..].to_string()),
        "ls" => Command::LS,
        _ => panic!("Unknown command: {}", input),
    }
}

fn parse_response(input: &str) -> Response {
    match input.chars().into_iter().next().unwrap() {
        'd' => {
            return Response::Directory(input[4..].to_string());
        }
        '0'..='9' => {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let size = parts[0].parse::<u32>().unwrap();
            let path = parts[1].to_string();
            return Response::File(path, size);
        }
        _ => panic!("Unknown response: {}", input),
    }
}

fn parse_line(line: &str) -> Line {
    let first_character = line.chars().into_iter().next().unwrap();
    match first_character {
        '$' => {
            // command
            let cmd = parse_cmd(&line[2..]);
            return Line::Command(cmd);
        }
        _ => {
            let response = parse_response(line);
            return Line::Response(response);
        }
    }
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Response(Response),
}

#[derive(Debug)]
enum Response {
    File(String, u32),
    Directory(String),
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS,
}

#[derive(Debug, Default)]
struct Node {
    children: HashMap<String, Node>,
    size: u32,
}

fn get_dirs_with_size_under(node: &Node, size: u32) -> Vec<(String, u32)> {
    let mut dirs: Vec<(String, u32)> = Vec::new();
    for (key, child) in &node.children {
        if child.children.is_empty() {
            // Is a file not a dir
            continue;
        }
        if child.size <= size {
            dirs.push((key.clone(), child.size));
        }
        dirs.append(&mut get_dirs_with_size_over(&child, size));
    }
    dirs
}

fn get_dirs_with_size_over(node: &Node, size: u32) -> Vec<(String, u32)> {
    let mut dirs: Vec<(String, u32)> = Vec::new();
    for (key, child) in &node.children {
        if child.children.is_empty() {
            // Is a file not a dir
            continue;
        }
        if child.size >= size {
            dirs.push((key.clone(), child.size));
        }
        dirs.append(&mut get_dirs_with_size_over(&child, size));
    }
    dirs
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            size: 0,
        }
    }

    fn update_size(&mut self) {
        for (_, child) in &mut self.children {
            child.update_size();
            self.size += child.size;
        }
    }
}

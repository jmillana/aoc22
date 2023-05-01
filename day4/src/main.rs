use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut contained_count = 0;
    let mut overlap_count = 0;
    for line in contents.lines() {
        let groups = line.split(",");
        let mut zones: Vec<Zone> = Vec::new();
        for group in groups {
            zones.push(Zone::build(group));
        }
        if zones[0].contains(&zones[1]) || zones[1].contains(&zones[0]) {
            contained_count += 1;
        }
        if zones[0].overlaps(&zones[1]) {
            overlap_count += 1;
        }
    }
    println!("{} zones contained", contained_count);
    println!("{} zones overlapped", overlap_count);
}

struct Zone {
    start: u32,
    end: u32,
}

impl Zone {
    fn build(zone: &str) -> Self {
        let mut zone = zone.split("-");
        let start = zone.next().unwrap().parse::<u32>().unwrap();
        let end = zone.next().unwrap().parse::<u32>().unwrap();
        return Zone { start, end };
    }

    fn contains(&self, other: &Zone) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: &Zone) -> bool {
        return self.start <= other.end && self.end >= other.start;
    }
}

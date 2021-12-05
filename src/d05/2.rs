use std::{cmp::max, collections::HashSet};

use regex::Regex;

#[derive(Debug)]
struct Board {
    pub remaining: HashSet<i32>,
    pub remaining_per_lane: Vec<HashSet<i32>>,
}

fn main() {
    let regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut single = HashSet::new();
    let mut double = HashSet::new();

    include_str!("input").split("\n").for_each(|line| {
        let captures = regex.captures(line.trim()).unwrap();
        let x1: i32 = captures[1].parse().unwrap();
        let y1: i32 = captures[2].parse().unwrap();
        let x2: i32 = captures[3].parse().unwrap();
        let y2: i32 = captures[4].parse().unwrap();
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let length = max((x2 - x1).abs(), (y2 - y1).abs());
        for idx in 0..=length {
            let coord = (x1 + idx * dx, y1 + idx * dy);
            if single.contains(&coord) {
                double.insert(coord);
            } else {
                single.insert(coord);
            }
        }
    });
    println!("{}", double.len())
}

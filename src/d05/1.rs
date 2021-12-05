use std::{
    cmp::{max, min},
    collections::HashSet,
};

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
        let range: Vec<(i32, i32)> = if x1 == x2 {
            let low_y = min(y1, y2);
            let hi_y = max(y1, y2);
            (low_y..=hi_y).map(|y| (x1, y)).collect()
        } else if y1 == y2 {
            let low_x = min(x1, x2);
            let hi_x = max(x1, x2);
            (low_x..=hi_x).map(|x| (x, y1)).collect()
        } else {
            Vec::new()
        };
        for coord in range {
            if single.contains(&coord) {
                double.insert(coord);
            } else {
                single.insert(coord);
            }
        }
    });
    println!("{}", double.len())
}

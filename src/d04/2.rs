use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    pub remaining: HashSet<i32>,
    pub remaining_per_lane: Vec<HashSet<i32>>,
    pub won: bool,
}

fn main() {
    let input: Vec<&str> = include_str!("input").split("\n").collect();
    let calls: Vec<i32> = input[0]
        .split(",")
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = input[1..]
        .chunks(6)
        .map(|lines| {
            let mut raw_board = [[0; 5]; 5];
            let mut remaining = HashSet::new();
            lines[1..].iter().enumerate().for_each(|(i, line)| {
                line.split(" ")
                    .filter(|v| !v.is_empty())
                    .enumerate()
                    .for_each(|(j, v)| {
                        let value = v.trim().parse().unwrap();
                        raw_board[i][j] = value;
                        remaining.insert(value);
                    });
            });
            let mut remaining_per_lane: Vec<HashSet<i32>> = raw_board
                .iter()
                .map(|lane| HashSet::from_iter(lane.iter().cloned()))
                .collect();
            for j in 0..5 {
                let mut vertical_lane = HashSet::new();
                for i in 0..5 {
                    vertical_lane.insert(raw_board[i][j]);
                }
                remaining_per_lane.push(vertical_lane)
            }
            Board {
                remaining,
                remaining_per_lane,
                won: false,
            }
        })
        .collect();
    let mut last = 0;
    for call in calls {
        for board in &mut boards {
            if board.won || !board.remaining.contains(&call) {
                continue;
            }
            board.remaining.remove(&call);
            for lane in &mut board.remaining_per_lane {
                if lane.remove(&call) && lane.len() == 0 {
                    last = board.remaining.iter().sum::<i32>() * call;
                    board.won = true;
                }
            }
        }
    }
    println!("{}", last)
}

fn main() {
    let values: Vec<i64> = include_str!("input")
        .split("\n")
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let count = values.windows(2).filter(|wnd| wnd[0] < wnd[1]).count();
    println!("{}", count);
}

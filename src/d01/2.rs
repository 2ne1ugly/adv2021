fn main() {
    let values: Vec<i64> = include_str!("input")
        .split("\n")
        .map(|v| v.trim().parse().unwrap())
        .collect();
    let sums: Vec<i64> = values.windows(3).map(|wnd| wnd.iter().sum()).collect();
    println!("{}", sums.windows(2).filter(|wnd| wnd[0] < wnd[1]).count());
}

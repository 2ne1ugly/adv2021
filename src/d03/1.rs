fn main() {
    let mut values: Vec<Vec<char>> = include_str!("input")
        .split("\n")
        .map(|v| v.trim().chars().collect())
        .collect();
    let length = values[0].len();
    let mut multiplier = (2 as i32).pow(length as u32 - 1);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..length {
        let zero_count = values.iter_mut().filter(|v| v[i] == '0').count();
        let one_count = values.len() - zero_count;
        if one_count > zero_count {
            gamma += multiplier;
        } else {
            epsilon += multiplier;
        }
        multiplier /= 2;
    }
    println!("{}", gamma * epsilon);
}

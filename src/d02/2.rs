use regex::Regex;

fn main() {
    let regex = Regex::new(r"^(\w+) (\d+)$").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    include_str!("input").split("\n").for_each(|v| {
        let captures = regex.captures(v.trim()).unwrap();
        let amount: i32 = captures[2].parse().unwrap();
        match &captures[1] {
            "forward" => {
                x += amount;
                y += aim * amount
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!(),
        };
    });

    println!("{}", x * y);
}

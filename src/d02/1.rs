use regex::Regex;

fn main() {
    let regex = Regex::new(r"^(\w+) (\d+)$").unwrap();
    let mut x = 0;
    let mut y = 0;
    include_str!("input").split("\n").for_each(|v| {
        let captures = regex.captures(v.trim()).unwrap();
        let amount: i32 = captures[2].parse().unwrap();
        match &captures[1] {
            "forward" => x += amount,
            "down" => y += amount,
            "up" => y -= amount,
            _ => panic!(),
        };
    });

    println!("{}", x * y);
}

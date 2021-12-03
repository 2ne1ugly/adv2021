fn main() {
    let values: Vec<&'static str> = include_str!("input")
        .split("\n")
        .map(|v| v.trim())
        .collect();

    fn calc_list(vec: Vec<&str>, idx: usize, fewer: bool) -> isize {
        if vec.len() == 1 {
            return isize::from_str_radix(vec[0], 2).unwrap();
        }
        let zero_count = vec
            .iter()
            .filter(|v| v.chars().nth(idx).unwrap() == '0')
            .count();
        let one_count = vec.len() - zero_count;
        let filter_char: char;
        if one_count == zero_count {
            if fewer {
                filter_char = '0';
            } else {
                filter_char = '1';
            }
        } else if one_count > zero_count {
            if fewer {
                filter_char = '0';
            } else {
                filter_char = '1';
            }
        } else {
            if fewer {
                filter_char = '1';
            } else {
                filter_char = '0';
            }
        }
        let new_vec: Vec<&str> = vec
            .iter()
            .filter(|v| v.chars().nth(idx).unwrap() == filter_char)
            .cloned()
            .collect();
        calc_list(new_vec, idx + 1, fewer)
    }
    let o2 = calc_list(values.clone(), 0, false);
    let co2 = calc_list(values.clone(), 0, true);

    println!("{}", o2 * co2);
}

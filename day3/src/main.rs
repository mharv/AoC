use std::fs;

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();
    let mut bit_counter = vec![0; 12];

    for binary_str in content_split.iter() {
        if binary_str.len() == 12 {
            for (i, char) in binary_str.chars().enumerate() {
                if char == '1' {
                    bit_counter[i] += 1
                }
            }
        }
    }

    let mut gamma_rate_bin_str = String::new();
    let mut epsilon_rate_bin_str = String::new();

    for total in bit_counter {
        if total > content_split.len()/2 {
            gamma_rate_bin_str.push('1');
            epsilon_rate_bin_str.push('0');
        } else {
            gamma_rate_bin_str.push('0');
            epsilon_rate_bin_str.push('1');
        }
    }

    let gamma_rate = i32::from_str_radix(&gamma_rate_bin_str, 2).expect("Not a binary number!");
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_bin_str, 2).expect("Not a binary number!");

    println!("power consumption is: {}", gamma_rate*epsilon_rate);
}

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

    // part 2
    // get most frequent at bit one
    let mut diag_report_for_oxy = content_split.clone();
    let mut bit_of_interest: usize = 0;

    while diag_report_for_oxy.len() > 1 {
        let mut one_occuring = 0;
        let mut zero_occuring = 0;
        for binary_str in diag_report_for_oxy.iter() {
            if binary_str.len() == 12 {
                if binary_str.chars().nth(bit_of_interest).unwrap() == '1' {
                    one_occuring += 1;
                } else {
                    zero_occuring += 1;
                }
            }
        }
        if one_occuring >= zero_occuring {
            diag_report_for_oxy = diag_report_for_oxy.into_iter().filter(|x| x.chars().nth(bit_of_interest).unwrap() == '1').collect();
        } else {
            diag_report_for_oxy = diag_report_for_oxy.into_iter().filter(|x| x.chars().nth(bit_of_interest).unwrap() == '0').collect();
        }
        bit_of_interest += 1;
    }

    println!("oxygen generator rating report length is: {}", diag_report_for_oxy.len());
    println!("oxygen generator rating is: {}", diag_report_for_oxy[0]);

    // get least frequent at bit one
    let mut diag_report_for_co2 = content_split.clone();
    let mut bit_of_interest: usize = 0;

    while diag_report_for_co2.len() > 1 {
        let mut one_occuring = 0;
        let mut zero_occuring = 0;
        for binary_str in diag_report_for_co2.iter() {
            if binary_str.len() == 12 {
                if binary_str.chars().nth(bit_of_interest).unwrap() == '1' {
                    one_occuring += 1;
                } else {
                    zero_occuring += 1;
                }
            }
        }
        if one_occuring < zero_occuring {
            diag_report_for_co2 = diag_report_for_co2.into_iter().filter(|x| x.chars().nth(bit_of_interest).unwrap() == '1').collect();
        } else {
            diag_report_for_co2 = diag_report_for_co2.into_iter().filter(|x| x.chars().nth(bit_of_interest).unwrap() == '0').collect();
        }
        bit_of_interest += 1;
    }

    println!("co2 scrubber rating report length is: {}", diag_report_for_co2.len());
    println!("co2 scrubber rating is: {}", diag_report_for_co2[0]);

    // get results
    let oxygen_generator_rating = i32::from_str_radix(&diag_report_for_oxy[0], 2).expect("Not a binary number!");
    let co2_scrubber_rating = i32::from_str_radix(&diag_report_for_co2[0], 2).expect("Not a binary number!");

    println!("life support rating is: {}", oxygen_generator_rating*co2_scrubber_rating);
}

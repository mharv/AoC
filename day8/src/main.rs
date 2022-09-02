use std::{fs, collections::HashMap};

fn main() {

    let path = String::from("./input.txt");
    // let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut totals = 0;

    for line in &content_split {
        let values: Vec<&str> = line.split(" | ").collect();
        for value in values[1].split_whitespace().collect::<Vec<&str>>().iter() {
            match value.len() {
                2 | 4 | 3 | 7 => totals += 1,
                _ => totals += 0,
            }
        }
    }
    println!("there is a total of {} 1's 4's 7's and 8's", totals);


    let mut grand_total = 0;

    for line in content_split {
        let mut look_up: HashMap<i32, String> = HashMap::new();
        let values: Vec<&str> = line.split(" | ").collect();

        while look_up.keys().len() < 10 {
            for value in values[0].split_whitespace().collect::<Vec<&str>>().iter() {
                // figure out each number
                if value.len() == 2 {
                    look_up.insert(1, sort_string_alphabetically(value.to_string()));
                }
                if value.len() == 4 {
                    look_up.insert(4, sort_string_alphabetically(value.to_string()));
                }
                if value.len() == 3 {
                    look_up.insert(7, sort_string_alphabetically(value.to_string()));
                }
                if value.len() == 7 {
                    look_up.insert(8, sort_string_alphabetically(value.to_string()));
                }
                if value.len() == 6 && look_up.contains_key(&4) && look_up.contains_key(&1){
                    // if length == 6 (0, 6, 9) and item contains 1 and 4, it is 9
                    // if length == 6 (0, 6, 9) and item doesnt contain 1, it is 6
                    // if length == 6 (0, 6, 9) and item doesnt contain 4, it is 0

                    let mut one_contained = 0;
                    let mut four_contained = 0;

                    let mut one_count = 0;
                    for segment in look_up[&1].chars() {
                        if value.contains(segment) {
                            one_count += 1;
                        }
                    }
                    if one_count == look_up[&1].len() {
                        one_contained = 1;
                    } else {
                        one_contained = 0;
                    }

                    let mut four_count = 0;
                    for segment in look_up[&4].chars() {
                        if value.contains(segment) {
                            four_count += 1;
                        }
                    }
                    if four_count == look_up[&4].len() {
                        four_contained = 1;
                    } else {
                        four_contained = 0;
                    }

                    if one_contained == 1 && four_contained == 0 { look_up.insert(0, sort_string_alphabetically(value.to_string())); }
                    if one_contained == 0 && four_contained == 0 { look_up.insert(6, sort_string_alphabetically(value.to_string())); }
                    if one_contained == 1 && four_contained == 1 { look_up.insert(9, sort_string_alphabetically(value.to_string())); }
                }
                if value.len() == 5 && look_up.contains_key(&1) && look_up.contains_key(&6){
                    // if length == 5 (2, 3, 5) and contains all but 2 of 9's segments, it is 2
                    // if length == 5 (2, 3, 5) and item contains 1, it is 3
                    // if length == 5 (2, 3, 5) and contains all but 1 of 9's segments, it is 5
                    let mut one_contained = 0;

                    let mut one_count = 0;
                    for segment in look_up[&1].chars() {
                        if value.contains(segment) {
                            one_count += 1;
                        }
                    }
                    if one_count == look_up[&1].len() {
                        one_contained = 1;
                    } else {
                        one_contained = 0;
                    }

                    let mut six_count = 0;
                    for segment in look_up[&6].chars() {
                        if value.contains(segment) {
                            six_count += 1;
                        }
                    }

                    if one_contained == 1 { look_up.insert(3, sort_string_alphabetically(value.to_string())); }
                    if one_contained == 0 && six_count == (look_up[&6].len()-2) { look_up.insert(2, sort_string_alphabetically(value.to_string())); }
                    if one_contained == 0 && six_count == (look_up[&6].len()-1) { look_up.insert(5, sort_string_alphabetically(value.to_string())); }
                }
            }
        }

        let mut look_up_reverse: HashMap<String, i32> = HashMap::new();
        for (k, v) in look_up.iter() {
            look_up_reverse.insert(v.clone(), k.clone());
        }

        let mut local_total = 0;
        for (i, value) in values[1].split_whitespace().collect::<Vec<&str>>().iter().enumerate() {
            // map each value to above
            match i {
                0 => local_total += 1000 * look_up_reverse[&sort_string_alphabetically(value.to_string())],
                1 => local_total += 100 * look_up_reverse[&sort_string_alphabetically(value.to_string())],
                2 => local_total += 10 * look_up_reverse[&sort_string_alphabetically(value.to_string())],
                3 => local_total += look_up_reverse[&sort_string_alphabetically(value.to_string())],
                _ => panic!("nup"),
            }
        }
        grand_total += local_total;
    }

    println!("grand total is: {}", grand_total);
}

fn sort_string_alphabetically(input: String) -> String {
    let mut temp_char_vec = input.chars().collect::<Vec<char>>();
    temp_char_vec.sort();
    temp_char_vec.into_iter().collect()
}

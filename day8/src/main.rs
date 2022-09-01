use std::{fs, collections::HashMap};

fn main() {

    // let path = String::from("./input.txt");
    let path = String::from("./test_input.txt");
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
                    look_up.insert(1, value.to_string());
                }
                if value.len() == 4 {
                    look_up.insert(4, value.to_string());
                }
                if value.len() == 3 {
                    look_up.insert(7, value.to_string());
                }
                if value.len() == 7 {
                    look_up.insert(8, value.to_string());
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

                    if one_contained == 1 && four_contained == 0 { look_up.insert(0, value.to_string()); }
                    if one_contained == 0 && four_contained == 0 { look_up.insert(6, value.to_string()); }
                    if one_contained == 1 && four_contained == 1 { look_up.insert(9, value.to_string()); }
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

                    if one_contained == 1 { look_up.insert(3, value.to_string()); }
                    if one_contained == 0 && six_count == (look_up[&6].len()-2) { look_up.insert(2, value.to_string()); }
                    if one_contained == 0 && six_count == (look_up[&6].len()-1) { look_up.insert(5, value.to_string()); }
                }
            }
        }

        let mut look_up_reverse: HashMap<String, i32> = HashMap::new();
        for (k, v) in look_up.iter() {
            println!("{} : {}", k, v);
            look_up_reverse.insert(v.clone(), k.clone());
        }

        let mut local_total = 0;
        for (i, value) in values[1].split_whitespace().collect::<Vec<&str>>().iter().enumerate() {
            let mut new_key = String::from("");
            // check if same segment
            for (k, _v) in look_up_reverse.iter() {
                let mut count = 0;
                for character in value.chars() {
                    if k.contains(character) {
                        count += 1;
                    }
                }
                if count == k.to_string().len() {
                    new_key = k.clone();
                }
            }
            // map each value to above
            match i {
                0 => local_total += 1000 * look_up_reverse[&new_key],
                1 => local_total += 100 * look_up_reverse[&new_key],
                2 => local_total += 10 * look_up_reverse[&new_key],
                3 => local_total += look_up_reverse[&new_key],
                _ => panic!("nup"),
            }
        }
        println!("local total for {:?} is: {}", &values[1], local_total);
        println!("{:?}", look_up_reverse);
        grand_total += local_total;
    }

    println!("grand total is: {}", grand_total);
}

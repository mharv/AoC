use std::{fs, collections::HashMap};

fn main() {
    // let path = String::from("./test_input.txt");
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    // get initial string
    let mut sequence: Vec<char> = content_split[0].chars().collect();

    let mut lookup: HashMap<String, &str> = HashMap::new();

    for row in content_split.iter() {
        if row.contains(" -> ") {
            let key = row.split(" -> ").collect::<Vec<&str>>()[0];
            let value = row.split(" -> ").collect::<Vec<&str>>()[1];

            lookup.insert(key.to_string(), value);
        }
    }

    for _ in 0..10 {
        sequence = do_step(&mut sequence, &lookup);
    }

    // count chars by occurence
    let mut total_counts: HashMap<char, i32> = HashMap::new();

    for character in sequence.iter() {
        if !total_counts.contains_key(character) {
            total_counts.insert(*character, 1);
        } else {
            total_counts.insert(*character, total_counts.get(character).unwrap() + 1);
        }
    }

    println!("{:?}", total_counts);

    let mut current_min = ('A', 0);
    let mut current_max = ('A', 0);
    // get max and min keys
    let mut i = 0;
    for (k, v) in total_counts.iter() {
        if i == 0 {
            current_min = (*k, *v);
            current_max = (*k, *v);
        } else {
            if v > &current_max.1 { current_max = (*k, *v) }
            if v < &current_min.1 { current_min = (*k, *v) }
        }

        i += 1;
    }

    println!("{} - {} = {}", current_max.1, current_min.1, current_max.1 - current_min.1);

}

fn do_step(sequence: &mut Vec<char>, lookup: &HashMap<String, &str>) -> Vec<char> {

    let mut result: Vec<char> = Vec::new();
    for (i, character) in sequence.iter().enumerate() {
        if i == sequence.len()-1 {
            result.push(*character);
            break;
        }
        let mut key = String::new();
        key.insert(0, *character);
        key.insert(1, sequence[i+1]);
        let value = lookup.get(&key).unwrap();
        result.push(*character);
        result.push(value.chars().next().unwrap());
    }
    result
}

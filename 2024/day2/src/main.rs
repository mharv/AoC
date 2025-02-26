use aoc_utils::get_input;

fn test_arr(arr: &Vec<i32>) -> bool {
    if !arr.is_sorted_by(|a, b| a > b) && !arr.is_sorted_by(|a, b| a < b) {
        return false;
    }
    let mut previous: &i32 = &0;
    for (i, v) in arr.iter().enumerate() {
        if i == 0 {
            previous = v;
            continue;
        } else {
            if (previous - v).abs() > 3 || (previous - v).abs() < 1 {
                return false;
            }
            previous = v;
        }
    }
    return true;
}

fn test_sub_arr(arr: &Vec<i32>) -> bool {
    let result = match test_arr(arr) {
        true => true,
        false => {
            for i in 0..arr.len() {
                let mut temp_arr = arr.clone();
                temp_arr.remove(i);
                if test_arr(&temp_arr) {
                    return true;
                }
            }
            return false;
        }
    };
    return result;
}

fn main() {
    let input_file = get_input(false);

    let mut processed: Vec<Vec<i32>> = Vec::new();

    for line in input_file.split("\n") {
        let temp_processed: Vec<&str> = line.split(" ").collect();
        let temp_processed: Vec<i32> = temp_processed
            .iter()
            .map(|item| item.parse().unwrap())
            .collect();
        processed.push(temp_processed);
    }

    // Part 1
    let mut count = 0;

    for arr in processed.iter() {
        match test_arr(&arr) {
            true => count += 1,
            false => (),
        }
    }

    println!("Part 1 result: {:?}", count);

    // Part 2

    let mut count_part_2 = 0;

    for arr in processed.iter() {
        match test_arr(&arr) {
            true => count_part_2 += 1,
            false => {
                if test_sub_arr(&arr) {
                    count_part_2 += 1
                }
            }
        }
    }

    println!("Part 1 result: {:?}", count_part_2);
}

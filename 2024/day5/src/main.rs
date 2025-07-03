use std::collections::HashMap;

use aoc_utils::get_input;

fn do_check(to_check: &Vec<i32>, lookup: &HashMap<i32, Vec<i32>>) -> bool {
    println!("{:?}", to_check);
    for (i, n) in to_check.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let look = &lookup.get(n);
        let arr = match look {
            Some(val) => val,
            _ => continue,
        };

        for x in 0..i + 1 {
            if arr.contains(&to_check[x]) {
                return false;
            }
        }
    }
    return true;
}

fn get_mid(arr: &Vec<i32>) -> i32 {
    return arr[arr.len() / 2];
}

fn main() {
    let ins = get_input(true);

    // parsing
    let mut lookup: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut to_check: Vec<Vec<i32>> = Vec::new();

    for line in ins.lines() {
        if line.contains("|") {
            let arr: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            if lookup.contains_key(&arr[0]) {
                lookup.get_mut(&arr[0]).unwrap().push(arr[1]);
            } else {
                lookup.insert(arr[0], Vec::new());
                lookup.get_mut(&arr[0]).unwrap().push(arr[1]);
            }
        }
        if line.contains(",") {
            to_check.push(Vec::new());
            let to_check_len = to_check.len();
            for i in line.split(",") {
                to_check
                    .get_mut(to_check_len - 1)
                    .unwrap()
                    .push(i.parse().unwrap());
            }
        }
    }

    println!("{:?}", lookup);

    let mut bools = Vec::new();

    for line in &to_check {
        bools.push(do_check(&line, &lookup));
    }

    let mut fails: Vec<Vec<i32>> = Vec::new();

    let mut result = 0;
    for (i, check) in bools.iter().enumerate() {
        if *check {
            result += get_mid(&to_check[i]);
        } else {
            fails.push(to_check[i].clone());
        }
    }
    println!("Part 1 : {}", result);

    println!("{:?}", fails);
}

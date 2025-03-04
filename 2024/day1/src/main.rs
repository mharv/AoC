use aoc_utils::get_input;
use std::collections::HashMap;
use std::iter::zip;
use std::str::FromStr;

fn main() {
    let mut part1_result = 0;
    let mut part2_result = 0;
    let message = get_input(false);
    let mut a = vec![];
    let mut b = vec![];

    for line in message.split("\n") {
        let processed: Vec<&str> = line.split("   ").collect();
        a.push(i32::from_str(processed[0]).unwrap());
        b.push(i32::from_str(processed[1]).unwrap());
    }

    // Part 1
    a.sort();
    b.sort();

    for (i, j) in zip(&a, &b) {
        part1_result += (i - j).abs();
    }

    println!("Part 1: {}", part1_result);

    // Part 2
    let mut b_count: HashMap<i32, i32> = HashMap::new();
    for i in b.clone().iter() {
        match b_count.get(i) {
            Some(num) => b_count.insert(*i, num + 1),
            None => b_count.insert(*i, 1),
        };
    }

    for i in a.iter() {
        match b_count.get(i) {
            Some(num) => part2_result += i * num,
            None => (),
        }
    }

    println!("Part 2: {}", part2_result);
}

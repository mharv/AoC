use aoc_utils::get_input;
use regex::Regex;

fn get_values(expression: String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut a: String = String::new();
    for c in expression.chars() {
        if c == ',' || c == ')' {
            result.push(a.parse().unwrap());
            a = String::new();
        }
        if c.is_numeric() {
            a.push(c);
        }
    }
    return result;
}

fn main() {
    let input = get_input(false);

    // let mut matches: Vec<String> = Vec::new();

    // let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    // let found = re.find_iter(&input);
    // for item in found {
    //     matches.push(item.as_str().to_string());
    // }

    // let mut result = 0;
    // for val in matches {
    //     let nums = get_values(val);
    //     result += nums.get(0).unwrap() * nums.get(1).unwrap();
    // }

    // println!("Part 1 is: {}", result);

    // part 2
    let mut matches: Vec<String> = Vec::new();

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    let found = re.find_iter(&input);
    for item in found {
        println!("{}", item.as_str());
        matches.push(item.as_str().to_string());
    }

    let mut result = 0;
    let mut toggle = true;
    for val in matches {
        match val.as_str() {
            "do()" => {
                toggle = true;
                continue;
            }
            "don't()" => {
                toggle = false;
                continue;
            }
            _ => (),
        }

        if toggle {
            let nums = get_values(val);
            result += nums.get(0).unwrap() * nums.get(1).unwrap();
        }
    }

    println!("Part 2 is: {}", result);
}

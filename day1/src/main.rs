use std::fs;

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut increase_count = 0;
    let mut previous_reading = 0;

    println!("content_split length: {}", content_split.len());

    for (i, _row) in content_split.iter().enumerate() {
        if i == content_split.len() - 2 {
            break;
        }
        let number1 = &content_split[i].parse::<i32>().expect("failed to convert to i32");
        let number2 = &content_split[i+1].parse::<i32>().expect("failed to convert to i32");
        let number3 = &content_split[i+2].parse::<i32>().expect("failed to convert to i32");

        let sum = number1 + number2 + number3;

        if i != 0 {
            if sum > previous_reading {
                increase_count += 1;
            }
        }
        previous_reading = sum;
    }

    println!("total increases: {}", increase_count)
}

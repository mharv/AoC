use std::fs;

fn main() {

    let path = String::from("./input.txt");
    //let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut totals = 0;

    for line in content_split {
        let values: Vec<&str> = line.split(" | ").collect();
        for value in values[1].split_whitespace().collect::<Vec<&str>>().iter() {
            match value.len() {
                2 | 4 | 3 | 7 => totals += 1,
                _ => totals += 0,
            }
        }
    }
    println!("there is a total of {} 1's 4's 7's and 8's", totals);
}

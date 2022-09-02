use std::fs;

fn main() {
    // let path = String::from("./input.txt");
    let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();
    println!("{:?}", content_split);
}

use std::fs;

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut horizontal_position = 0;
    let mut depth_position = 0;

    let mut aim = 0;

    for (_, command) in content_split.iter().enumerate() {
        let action: Vec<&str> = command.split(" ").collect();

        if action.len() > 0 {
            match action[0] {
                "forward" => {
                    horizontal_position += action[1].parse::<i32>().unwrap();
                    if aim != 0 {
                        depth_position += aim * action[1].parse::<i32>().unwrap();
                    }
                },
                "up" => aim -= action[1].parse::<i32>().unwrap(),
                "down" => aim += action[1].parse::<i32>().unwrap(),
                _ => panic!("wrong command given!"),

            }
        }
    }
    println!("multiplied height and depth: {}", horizontal_position * depth_position);
}

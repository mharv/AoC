use std::fs;

fn main() {
    let path = String::from("./input.txt");
    //let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut horizontal_positions: Vec<i32> = Vec::new();
    for item in content_split[0].split(",").collect::<Vec<&str>>().iter() {
        horizontal_positions.push(item.parse().expect("not a number"))
    }

    let max_horizontal_position = horizontal_positions.iter().max().unwrap();
    println!("max horizontal position is: {}", max_horizontal_position);

    let mut lowest_cost_position: i32 = 0;
    let mut lowest_cost_position_cost: i32 = 0;
    let mut last_position_cost: i32 = 0;

    for position in 0..=*max_horizontal_position {
        let current_position_cost = get_cost_of_position(&horizontal_positions, position);
        if position == 0 {
            last_position_cost = current_position_cost;
        }

        if current_position_cost < last_position_cost {
            lowest_cost_position = position;
            lowest_cost_position_cost = current_position_cost;
        }
        println!("total cost for position {} is: {}", position, current_position_cost);
        last_position_cost = current_position_cost;
    }

    println!("lowest cost position {} with cost of: {}", lowest_cost_position, lowest_cost_position_cost);
}

fn get_cost_of_position(positions: &Vec<i32>, n: i32) -> i32 {

    let mut total_cost = 0;

    for position in positions.iter() {
        let mut j = 1;
        for _ in 0..(position - n ).abs() {
            total_cost += j;
            j += 1;
        }
    }
    total_cost
}

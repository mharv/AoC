use std::fs;

fn main() {
    let path = String::from("./test_input.txt");
    // let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    // get size of matrix
    let mut x_size = 0;
    let mut y_size = 0;

    for line in content_split.iter() {
        if !line.is_empty() && line.contains("fold along") {
            if line.contains("y") {
                y_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                y_size *= 2;

            }
            if line.contains("x") {
                x_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                x_size *= 2;
            }
        }
    }

    let matrix = vec![vec!["."; x_size]; y_size];

    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!("{}", cell);
        }
        print!("\n");
    }

}

use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    id: i32,
    start_point: Point,
    end_point: Point,
}

fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut lines: Vec<Line> = Vec::new();

    for (i, line) in content_split.iter().enumerate() {

        let points: Vec<&str> = line.split(" -> ").collect();
        let temp_start_point: Vec<&str> = points[0].split(",").collect();
        let sp_x = temp_start_point[0].parse().expect("not a number");
        let sp_y = temp_start_point[1].parse().expect("not a number");
        let temp_end_point: Vec<&str> = points[1].split(",").collect();
        let ep_x = temp_end_point[0].parse().expect("not a number");
        let ep_y = temp_end_point[1].parse().expect("not a number");

        let start_point = Point { x: sp_x, y: sp_y };
        let end_point = Point { x: ep_x , y: ep_y };

        let temp_line = Line { id: i as i32, start_point, end_point };
        lines.push(temp_line);
    }


    for line in lines.iter() {
        println!("{:?}", line);
    }
}

use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    id: i32,
    start_point: Point,
    end_point: Point,
}

#[derive(Debug)]
struct Grid {
    matrix: Vec<Vec<u8>>,
}

impl Grid {
    fn new(grid_size: usize) -> Self {
        let matrix = vec![vec![0; grid_size]; grid_size];
        Grid { matrix }
    }

    fn mark_line(&mut self, line: &Line) {
        // only horizontal or vertical lines
        if line.start_point.x == line.end_point.x || line.start_point.y == line.end_point.y {
            for y in 0..self.matrix.len() {
                for x in 0..self.matrix[y].len() {
                    if y >= line.start_point.y && y <= line.end_point.y && x >= line.start_point.x && x <= line.end_point.x {
                        self.matrix[y][x] += 1;
                    }
                }
            }
        } else {
            // line must be diagonal
            if line.start_point.x < line.end_point.x && line.start_point.y < line.end_point.y {
                for (i, x) in ((line.start_point.x)..=line.end_point.x).enumerate() {
                    self.matrix[line.start_point.y + i][x] += 1;
                }
            }
            if line.start_point.x > line.end_point.x && line.start_point.y < line.end_point.y {
                for (i, x) in ((line.end_point.x)..=line.start_point.x).rev().enumerate() {
                    self.matrix[line.start_point.y + i][x] += 1;
                }
            }
            if line.start_point.x < line.end_point.x && line.start_point.y > line.end_point.y {
                for (i, x) in ((line.start_point.x)..=line.end_point.x).enumerate() {
                    self.matrix[line.start_point.y - i][x] += 1;
                }
            }
        }
    }

    fn check_for_overlaps(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] >= 2 {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_grid(&self) {
        for row in self.matrix.iter() {
            for item in row {
                if item == &0 {
                    print!(".");
                } else {
                    print!("{}", item);
                }
            }
            print!("\n");
        }
    }
}

const GRID_SIZE: usize = 1000;
const TEST_GRID_SIZE: usize = 10;

fn main() {
    let path = String::from("./input.txt");
    let mut grid = Grid::new(GRID_SIZE);

    // let path = String::from("./test_input.txt");
    // let mut grid = Grid::new(TEST_GRID_SIZE);

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

        let start_point: Point;
        let end_point: Point;

        // will this make the start point values always be lower
        // than the endpoint ones?
        if sp_y > ep_y || sp_x > ep_x {
            start_point = Point { x: ep_x, y: ep_y };
            end_point = Point { x: sp_x , y: sp_y };
        } else {
            start_point = Point { x: sp_x, y: sp_y };
            end_point = Point { x: ep_x , y: ep_y };
        }

        let temp_line = Line { id: i as i32, start_point, end_point };
        lines.push(temp_line);
    }



    for line in lines.iter() {
        grid.mark_line(&line);
    }
    let result = grid.check_for_overlaps();
    println!("there are {} overlaps", result);

    // grid.print_grid();
}

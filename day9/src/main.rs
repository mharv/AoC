use std::{fs, collections::VecDeque};


#[derive(Debug)]
struct Grid {
    matrix: Vec<Vec<u8>>,
    low_points: Vec<Point>,
    basins: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Grid {
    fn new(input: String) -> Self {

        let content_split: Vec<&str> = input.trim().split("\n").collect();
        let mut matrix = vec![vec![0; content_split[0].len()]; content_split.len()];

        for (y, line) in content_split.iter().enumerate() {
            for (x, digit) in line.chars().enumerate() {
                matrix[y][x] = digit.to_digit(10).expect("not a number!") as u8;
            }
        }

        Grid { matrix, low_points: Vec::new(), basins: Vec::new() }
    }

    fn print_grid(&self) {
        for y in self.matrix.iter() {
            for x in y.iter() {
                print!("{}", x);
            }
            print!("\n");
        }
    }

    fn print_grid_dimensions(&self) {
        println!("x: {}, y: {}", self.matrix[0].len(), self.matrix.len());
    }

    fn find_low_points(&mut self) -> i32 {
        let mut total_low_points: i32 = 0;

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {

                let mut temp_items: Vec<u8> = Vec::new();
                // check left
                if x > 0 {
                    temp_items.push(self.matrix[y][x - 1]);
                }
                // check right
                if x < row.len()-1 {
                    temp_items.push(self.matrix[y][x + 1]);
                }
                // check up
                if y > 0 {
                    temp_items.push(self.matrix[y - 1][x]);
                }
                // check down
                if y < self.matrix.len()-1 {
                    temp_items.push(self.matrix[y + 1][x]);
                }

                let mut count = 0;
                for temp_item in temp_items.iter() {
                    if item < temp_item {
                        count += 1;
                    }
                }

                if temp_items.len() == count {
                    total_low_points += *item as i32 + 1;
                    self.low_points.push(Point { x, y })
                }
            }
        }

        total_low_points
    }

    fn find_basins(&mut self) {
        for low_point in self.low_points.iter() {
            // cant use recursive function like this
            // flood_fill(&self.matrix, point, &mut count);
            //
            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(low_point.clone());
            self.matrix[low_point.y][low_point.x] = 9;
            let mut count = 1;

            while queue.len() > 0 {
                let point = queue.pop_front().unwrap();

                // check left
                if point.x > 0 {
                    if self.matrix[point.y][point.x - 1] != 9 {
                        queue.push_back(Point { x: point.x - 1, y: point.y });
                        self.matrix[point.y][point.x - 1] = 9;
                        count += 1;
                    }
                }
                // check right
                if point.x < self.matrix[0].len()-1 {
                    if self.matrix[point.y][point.x + 1] != 9 {
                        queue.push_back(Point { x: point.x + 1, y: point.y });
                        self.matrix[point.y][point.x + 1] = 9;
                        count += 1;
                    }
                }
                // check up
                if point.y > 0 {
                    if self.matrix[point.y - 1][point.x] != 9 {
                        queue.push_back(Point { x: point.x, y: point.y - 1 });
                        self.matrix[point.y - 1][point.x] = 9;
                        count += 1;
                    }
                }
                // check down
                if point.y < self.matrix.len()-1 {
                    if self.matrix[point.y + 1][point.x] != 9 {
                        queue.push_back(Point { x: point.x, y: point.y + 1 });
                        self.matrix[point.y + 1][point.x] = 9;
                        count += 1;
                    }
                }
            }

            self.basins.push(count);
        }
        self.basins.sort();
        self.basins.reverse();
        println!("3 largest basins multiplied: {}", self.basins[0] * self.basins[1] * self.basins[2]);
    }
}

// cant use recursive function like this
fn flood_fill(matrix: &Vec<Vec<u8>>, point: &Point, count: &mut i32) {
    if matrix[point.y][point.x] != 9 {
        *count += 1;

        if point.x > 0 {
            flood_fill(matrix, &Point { x: point.x-1, y: point.y }, count);
        }
        // check right
        if point.x < matrix[0].len()-1 {
            flood_fill(matrix, &Point { x: point.x+1, y: point.y }, count);
        }
        // check up
        if point.y > 0 {
            flood_fill(matrix, &Point { x: point.x, y: point.y-1 }, count);
        }
        // check down
        if point.y < matrix.len()-1 {
            flood_fill(matrix, &Point { x: point.x, y: point.y+1 }, count);
        }
    }
}

fn main() {

    // let path = String::from("./test_input.txt");
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");

    let mut grid = Grid::new(content);

    grid.print_grid_dimensions();

    let result = grid.find_low_points();
    println!("low points added together = {}", result);
    grid.find_basins();

}

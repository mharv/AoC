use std::fs;


#[derive(Debug)]
struct Grid {
    matrix: Vec<Vec<u8>>,
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

        Grid { matrix }
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

    fn find_low_points(&self) -> i32 {
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
                }
            }
        }

        total_low_points
    }
}

fn main() {

    // let path = String::from("./test_input.txt");
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");

    let grid = Grid::new(content);

    // grid.print_grid();
    grid.print_grid_dimensions();

    let result = grid.find_low_points();
    println!("low points added together = {}", result);

}

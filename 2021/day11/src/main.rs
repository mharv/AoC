use std::fs;

fn main() {
    let path = String::from("./input.txt");
    // let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut cavern = Cavern::new(content_split);
    // cavern.perform_n_steps(100);
    println!("total flash count: {}", cavern.flash_count);
    cavern.display_cavern();
    println!("first synchronous flash at step: {}", cavern.first_synchronous_flash());

}

struct Cavern {
    matrix: Vec<Vec<u8>>,
    flash_marker: Vec<Vec<u8>>,
    flash_count: i32,
    synchronous_flash_flag: u8,
}

impl Cavern {
    fn new(input: Vec<&str>) -> Self {

        let mut matrix = vec![vec![0; input[0].len()]; input.len()];
        let flash_marker = vec![vec![0; input[0].len()]; input.len()];

        for (y, row) in input.iter().enumerate() {
            for (x, energy_level) in row.chars().enumerate() {
                matrix[y][x] = energy_level.to_digit(10).unwrap() as u8;
            }
        }

        Cavern { matrix, flash_marker, flash_count: 0, synchronous_flash_flag: 0 }
    }

    fn display_cavern(&self) {
        for (_, row) in self.matrix.iter().enumerate() {
            for (_, energy_level) in row.iter().enumerate() {
                print!("{}", energy_level);
            }
            print!("\n");
        }
    }

    fn first_synchronous_flash(&mut self) -> i32 {
        let mut step_count = 0;
        while self.synchronous_flash_flag != 1 {
            self.perform_n_steps(1);
            step_count += 1;
        }
        step_count
    }

    fn perform_n_steps(&mut self, n: i32) {
        if n < 1 { return };

        for step in 0..n {
            let matrix_length = self.matrix.len();

            // increment everything
            for y in 0..matrix_length {
                let row_length = self.matrix[y].len();
                for x in 0..row_length {
                    self.matrix[y][x] += 1;
                }
            }

            // check for flashes
            // get greater than nine count
            let mut gt_nine_count = get_flash_count(&self.matrix);
            let mut mark_count = get_marker_count(&self.flash_marker);

            while gt_nine_count != mark_count {
                for y in 0..matrix_length {
                    let row_length = self.matrix[y].len();
                    for x in 0..row_length {
                        // print!("{}, {}: {}  ", x, y, energy_level);

                        let up_limit; let down_limit; let left_limit; let right_limit;

                        // check left
                        if x == 0 { left_limit = 0; } else { left_limit = 1; }
                        // check right
                        if x == row_length-1 { right_limit = 0; } else { right_limit = 1; }
                        // check up
                        if y == 0 { up_limit = 0; } else { up_limit = 1; }
                        // check down
                        if y == matrix_length-1 { down_limit = 0; } else { down_limit = 1; }


                        if self.matrix[y][x] > 9 && self.flash_marker[y][x] == 0 {
                            self.flash_marker[y][x] = 1;

                            for b in (y-up_limit)..=(y+down_limit) {
                                for a in (x-left_limit)..=(x+right_limit) {
                                    self.matrix[b][a] += 1;
                                }
                            }
                        }
                    }
                }
                gt_nine_count = get_flash_count(&self.matrix);
                mark_count = get_marker_count(&self.flash_marker);
            }

            // clear flash marker
            for y in 0..self.flash_marker.len() {
                for x in 0..self.flash_marker[y].len() {
                    self.flash_marker[y][x] = 0;
                }
            }

            // reset greater than 9
            let mut flash_count = 0;
            for y in 0..matrix_length {
                let row_length = self.matrix[y].len();
                for x in 0..row_length {
                    if self.matrix[y][x] > 9 {
                        self.flash_count += 1;

                        self.matrix[y][x] = 0;
                        flash_count += 1;
                    }
                }
            }
            if flash_count == self.matrix.len() * self.matrix[0].len() {
                self.synchronous_flash_flag = 1;
            }
        }
    }
}

fn get_flash_count(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut count = 0;
    for y in 0..matrix.len() {
        let row_length = matrix[y].len();
        for x in 0..row_length {
            if matrix[y][x] > 9 {
                count += 1;
            }
        }
    }
    count
}

fn get_marker_count(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut count = 0;
    for y in 0..matrix.len() {
        let row_length = matrix[y].len();
        for x in 0..row_length {
            if matrix[y][x] == 1 {
                count += 1;
            }
        }
    }
    count
}


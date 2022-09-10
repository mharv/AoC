use std::fs;


struct Paper {
    matrix: Vec<Vec<String>>,
}

impl Paper {
    fn new(input: Vec<&str>) -> Self {

        let mut x_size = 0;
        let mut y_size = 0;

        let mut y_fold = 0;
        let mut x_fold = 0;

        for line in input.iter() {
            if !line.is_empty() && line.contains("fold along") {
                if line.contains("y") {
                    y_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                    y_fold = y_size;
                    y_size *= 2;
                    y_size += 1;

                }
                if line.contains("x") {
                    x_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                    x_fold = x_size;
                    x_size *= 2;
                    x_size += 1;
                }
            }

            if x_size != 0 && y_size != 0 {
                break;
            }
        }

        let matrix = vec![vec![".".to_string(); x_size]; y_size];
        Paper { matrix }
    }

    fn display_paper(&self) {
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                print!("{}", cell);
            }
            print!("\n");
        }
    }

    fn display_size(&self) {
        println!("matrix x size is {}", self.matrix[0].len());
        println!("matrix y size is {}", self.matrix.len());
    }

    fn mark_coordinates(&mut self, input: Vec<&str>) {

        for line in input.iter() {
            if !line.is_empty() && line.contains("fold along") {
                if line.contains("y") {

                }
                if line.contains("x") {

                }
            }
        }
    }
}

fn main() {
    let path = String::from("./test_input.txt");
    // let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let paper = Paper::new(content_split);

    paper.display_size();
    paper.display_paper();




    // mark the y fold.
}

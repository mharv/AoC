use std::fs;


struct Paper {
    matrix: Vec<Vec<String>>,
    folds: Vec<Fold>
}

struct Fold {
    axis: String,
    index: i32,
}

impl Paper {
    fn new(input: &Vec<&str>) -> Self {

        let mut folds = Vec::new();
        let mut x_size = 0;
        let mut y_size = 0;

        let mut x = 0;
        let mut y = 0;

        for line in input.iter() {
            if !line.is_empty() && line.contains("fold along") {
                if line.contains("y") {
                    y_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                    folds.push(Fold { axis: "y".to_string(), index: y_size });
                    y_size *= 2;
                    y_size += 1;
                    if y == 0 { y = y_size }

                }
                if line.contains("x") {
                    x_size = line.split("=").collect::<Vec<&str>>()[1].parse().expect("not a number");
                    folds.push(Fold { axis: "x".to_string(), index: x_size });
                    x_size *= 2;
                    x_size += 1;
                    if x == 0 { x = x_size }
                }
            }
        }

        let matrix = vec![vec![".".to_string(); x as usize]; y as usize];
        Paper { matrix, folds }
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

    fn mark_coordinates(&mut self, input: &Vec<&str>) {

        for line in input.iter() {
            if !line.is_empty() && line.contains(",") {
                let x = line.split(",").collect::<Vec<&str>>()[0];
                let x: usize = x.parse().unwrap();
                let y = line.split(",").collect::<Vec<&str>>()[1];
                let y: usize = y.parse().unwrap();

                self.matrix[y][x] = "#".to_string();
            }
        }
    }

    fn count_hashes(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == "#" {
                    count += 1;
                }
            }
        }
        count
    }

    fn fold_along_axis(&mut self, axis: String) {

        let mut list_of_marks: Vec<(usize, usize)> = Vec::new();
        let mut count_to_drop = 0;

        if axis == "y" {
            // gather marks
            'outer: for (y, row) in self.matrix.iter().enumerate() {
                count_to_drop += 1;
                for (x, _) in row.iter().enumerate() {
                    if y == self.matrix.len()-1-y {
                        break 'outer;
                    }
                    if self.matrix[self.matrix.len()-1-y][x] == "#" {
                        list_of_marks.push((x, y))
                    }
                }
            }
            // remove old rows
            for _ in 0..count_to_drop {
                self.matrix.pop();
            }

        } else {
            // gather marks
            for (y, row) in self.matrix.iter().enumerate() {
                count_to_drop = 0;
                for (x, _) in row.iter().enumerate() {
                    count_to_drop += 1;
                    if x == row.len()-1-x {
                        break;
                    }
                    if row[row.len()-1-x] == "#" {
                        list_of_marks.push((x, y))
                    }
                }
            }
            // remove old items
            for row_index in 0..self.matrix.len() {
                for _ in 0..count_to_drop {
                    self.matrix[row_index].pop();
                }
            }
        }

        for mark in list_of_marks {
            self.matrix[mark.1][mark.0] = "#".to_string();
        }
    }
}

fn main() {
    // let path = String::from("./test_input.txt");
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut paper = Paper::new(&content_split);
    paper.mark_coordinates(&content_split);

    for i in 0..paper.folds.len() {
        paper.fold_along_axis(paper.folds[i].axis.to_string());
    }

    paper.display_paper();

    println!("number of hashes: {}", paper.count_hashes());
}

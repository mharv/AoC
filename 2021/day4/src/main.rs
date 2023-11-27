use std::fs;
#[derive(Debug)]
struct Board {
    number: i32,
    rows: Vec<Vec<i32>>,
    marks: Vec<Vec<u8>>,
}

impl Board {
    fn new(number: i32, raw_board_input: &str ) -> Self {
        let content_split: Vec<&str> = raw_board_input.trim().split("\n").collect();
        let mut rows = Vec::new();

        for (_, row) in content_split.iter().enumerate() {
            let mut temp_row = Vec::new();
            for val in row.split_whitespace().collect::<Vec<&str>>() {
                temp_row.push(val.parse::<i32>().expect("not a number"));
            }
            rows.push(temp_row);
        }

        let mut marks = Vec::new();
        for _ in 0..rows.len() {
            marks.push(vec![0; 5])
        }

        Board { number, rows, marks }
    }

    fn check_for_number_and_mark(&mut self, n: &i32) {

        for (y, row) in self.rows.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if val == n {
                    self.marks[y][x] = 1;
                }
            }
        }
    }

    fn check_for_win_condition(&mut self) -> (bool, i32) {

        // check columns
        for col_index in 0..self.marks[0].len() {
            let mut count = 0;
            for row in self.marks.iter() {
                if row[col_index] != 0 {
                    count += 1;
                }
            }
            if count == 5 {
                return (true, self.number);
            }
        }
        // check rows
        for row in self.marks.iter() {
            if !row.contains(&0) {
                return (true, self.number);
            }
        }
        return (false, self.number);
    }

    fn calculate_unmarked_total(&self) -> i32 {
        let mut total = 0;
        for (y, row) in self.marks.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if val == &0 {
                    total += self.rows[y][x];
                }
            }
        }
        total
    }
}



fn main() {
    let path = String::from("./input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n\n").collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut drawn_cards: Vec<i32> = Vec::new();

    for drawn_val in content_split[0].split(',').collect::<Vec<&str>>() {
        drawn_cards.push(drawn_val.parse::<i32>().expect("not a number"));
    }

    let mut board_number = 0;
    for row in content_split.iter().skip(1) {
        if row.len() > 0 {
            let temp_board = Board::new(board_number, row);
            boards.push(temp_board);
            board_number += 1;
        }
    }

    // find first winner and calculate score
    'outer: for drawn_val in drawn_cards.iter() {
        for board in boards.iter_mut() {
            board.check_for_number_and_mark(drawn_val);
            let winner = board.check_for_win_condition();
            if winner.0 {
                println!("winning board is: {}", winner.1);
                let score = board.calculate_unmarked_total() * drawn_val;
                println!("winning score for board {} is {} with current drawn value of {}",winner.1, score, drawn_val);
                break 'outer
            }
        }
    }

    // find last winner and calculate score

    let mut won_boards: Vec<i32> = Vec::new();
    let boards_length = boards.len();

    'outer: for drawn_val in drawn_cards.iter() {
        for board in boards.iter_mut() {
            if !won_boards.contains(&board.number) {
                board.check_for_number_and_mark(drawn_val);
                let winner = board.check_for_win_condition();
                if winner.0 {
                    won_boards.push(board.number);

                    if won_boards.len() == boards_length {

                        println!("number of won boards: {}", won_boards.len());
                        // won_boards.sort();
                        println!("{:?}", won_boards);
                        println!("last winning board is: {}", board.number);
                        let score = board.calculate_unmarked_total() * drawn_val;
                        println!("winning score for last board {} is {} with current drawn value of {}", board.number, score, drawn_val);
                        println!("{:?}", board.rows);
                        println!("{:?}", board.marks);
                        break 'outer
                    }
                }
            }
        }
    }
}

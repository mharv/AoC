use aoc_utils::{self, get_input};

fn parse_input(input_file: String) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    for c in input_file.chars() {
        if c == '\n' {
            output.push(temp);
            temp = Vec::new();
            continue;
        }
        temp.push(c.to_string());
    }
    output.push(temp);
    return output;
}

fn find_horizontal(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for row in grid {
        for (i, v) in row.iter().enumerate() {
            if row.len() == i + 3 {
                break;
            }
            let temp = format!("{}{}{}{}", v, row[i + 1], row[i + 2], row[i + 3]);
            if temp == "XMAS" || temp == "SAMX" {
                count += 1;
            }
        }
    }
    return count;
}

fn find_vertical(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if grid.len() == y + 3 {
                break;
            }
            let temp = format!(
                "{}{}{}{}",
                c,
                grid[y + 1][x],
                grid[y + 2][x],
                grid[y + 3][x]
            );
            if temp == "XMAS" || temp == "SAMX" {
                count += 1;
            }
        }
        if grid.len() == y + 3 {
            break;
        }
    }
    return count;
}

fn find_left_to_right_diag(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if grid.len() == y + 3 || row.len() == x + 3 {
                break;
            }
            let temp = format!(
                "{}{}{}{}",
                c,
                grid[y + 1][x + 1],
                grid[y + 2][x + 2],
                grid[y + 3][x + 3]
            );
            if temp == "XMAS" || temp == "SAMX" {
                count += 1;
            }
        }
        if grid.len() == y + 3 {
            break;
        }
    }
    return count;
}

fn find_right_to_left_diag(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if x < 3 {
                continue;
            }
            if grid.len() == y + 3 || row.len() == x {
                break;
            }
            let temp = format!(
                "{}{}{}{}",
                c,
                grid[y + 1][x - 1],
                grid[y + 2][x - 2],
                grid[y + 3][x - 3]
            );
            if temp == "XMAS" || temp == "SAMX" {
                count += 1;
            }
        }
        if grid.len() == y + 3 {
            break;
        }
    }
    return count;
}

fn find_mas(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if grid.len() == y + 2 || row.len() == x + 2 {
                break;
            }
            let temp1 = format!("{}{}{}", grid[y][x], grid[y + 1][x + 1], grid[y + 2][x + 2]);
            let temp2 = format!("{}{}{}", grid[y + 2][x], grid[y + 1][x + 1], grid[y][x + 2]);
            if (temp1 == "MAS" || temp1 == "SAM") && (temp2 == "MAS" || temp2 == "SAM") {
                count += 1;
                println!("{}, {}", temp1, temp2);
                println!("~~~");
                println!(
                    "{}",
                    format!("{}{}{}", grid[y][x], grid[y][x + 1], grid[y][x + 2])
                );
                println!(
                    "{}",
                    format!(
                        "{}{}{}",
                        grid[y + 1][x],
                        grid[y + 1][x + 1],
                        grid[y + 1][x + 2]
                    )
                );
                println!(
                    "{}",
                    format!(
                        "{}{}{}",
                        grid[y + 2][x],
                        grid[y + 2][x + 1],
                        grid[y + 2][x + 2]
                    )
                );
                println!("---")
            }
        }
        if grid.len() == y + 2 {
            break;
        }
    }
    return count;
}

fn main() {
    let input_file = get_input(false);
    let grid = parse_input(input_file);
    let mut count = 0;
    count += find_horizontal(&grid);
    count += find_vertical(&grid);
    count += find_left_to_right_diag(&grid);
    count += find_right_to_left_diag(&grid);

    println!("Part 1: {}", count);

    println!("Part 2: {}", find_mas(&grid));
}

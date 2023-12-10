mod utils;

const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const USE_EXAMPLE: bool = false;

fn get_input() -> Result<String, std::io::Error> {
    if USE_EXAMPLE {
        Ok(EXAMPLE.to_string())
    } else {
        utils::read_file("input.txt")
    }
}

fn main() {
    let s = match get_input() {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };
    let lines = utils::read_lines(&s);

    let mut result: u32 = 0;
    for line in lines {
        let numbers: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            let temp = format!("{}{}", first, last);
            if let Ok(parsed) = temp.parse::<u32>() {
                result += parsed;
            } else {
                eprintln!("Error parsing number: {}", temp);
            }
        }
    }
    println!("result: {}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = utils::read_file("input.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_lines() {
        let lines = utils::read_lines(EXAMPLE);
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_get_input() {
        let result = get_input();
        assert!(result.is_ok());
    }
}

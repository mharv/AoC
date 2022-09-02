use std::fs;

struct Character {
    character: char,
    closing_character: char,
}

fn main() {
    let path = String::from("./input.txt");
    // let path = String::from("./test_input.txt");
    let content = fs::read_to_string(path).expect("file was not read");
    let content_split: Vec<&str> = content.trim().split("\n").collect();

    let mut stack: Vec<Character> = Vec::new();
    let mut illegal_chars: Vec<char> = Vec::new();

    for line in content_split.iter() {
        for character in line.chars() {

            match character {
                '[' => stack.push(Character { character, closing_character: ']' }),
                '<' => stack.push(Character { character, closing_character: '>' }),
                '{' => stack.push(Character { character, closing_character: '}' }),
                '(' => stack.push(Character { character, closing_character: ')' }),
                ']' => if character_check(&mut stack, &mut illegal_chars, character) == 0 { break; },
                '>' => if character_check(&mut stack, &mut illegal_chars, character) == 0 { break; },
                '}' => if character_check(&mut stack, &mut illegal_chars, character) == 0 { break; },
                ')' => if character_check(&mut stack, &mut illegal_chars, character) == 0 { break; },
                _ => (),
            }
        }
    }

    let mut total_score = 0;

    for character in illegal_chars.iter() {
        match character {
                ']' => total_score += 57,
                '>' => total_score += 25137,
                '}' => total_score += 1197,
                ')' => total_score += 3,
                _ => (),
        }
    }

    println!("final score: {}", total_score);
}

fn character_check(stack: &mut Vec<Character>, illegals: &mut Vec<char>, character: char) -> i32 {
    if character == stack[stack.len() - 1].closing_character {
        stack.pop();
        return 1;
    } else {
        illegals.push(character);
        return 0;
    }
}

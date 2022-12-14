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
    let mut incomplete_line_indexes: Vec<usize> = Vec::new();

    for (index, line) in content_split.iter().enumerate() {
        for (i, character) in line.chars().enumerate() {
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
            if i == line.len() - 1 {
                incomplete_line_indexes.push(index);
            }
        }
    }

    let mut scores: Vec<i64> = Vec::new();

    for i in incomplete_line_indexes.iter() {
        let mut stack: Vec<Character> = Vec::new();
        for character in content_split[*i].chars() {
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

        let mut local_score: i64 = 0;
        for character in stack.iter().rev() {
            local_score *= 5;
            match character.closing_character {
                ']' => local_score += 2,
                '>' => local_score += 4,
                '}' => local_score += 3,
                ')' => local_score += 1,
                _ => (),
            };
        }
        scores.push(local_score);

        stack.clear();
    }

    scores.sort();

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

    println!("final score part 1: {}", total_score);
    println!("final score part 2: {:?}", scores[scores.len()/2]);
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

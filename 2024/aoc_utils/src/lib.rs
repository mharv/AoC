use std::fs;
use std::path;

pub fn get_input(test: bool) -> String {
    let mut file = "";
    match test {
        true => file = "./data/sample_input.txt",
        false => file = "./data/input.txt",
    }
    let absolute = path::absolute(file).unwrap();
    println!("{}", absolute.to_str().unwrap());
    fs::read_to_string(&absolute).unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}

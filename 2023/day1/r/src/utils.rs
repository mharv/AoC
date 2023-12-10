use std::fs;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn read_lines(s: &str) -> Vec<String> {
    s.lines().map(String::from).collect()
}

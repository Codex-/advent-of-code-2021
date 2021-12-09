use std::fs;
use std::path;

pub fn read_lines_from_input(filename: &str) -> Vec<String> {
    let input_path = path::Path::new("./fixtures").join(path::Path::new(filename));
    let contents = fs::read_to_string(input_path).unwrap();
    contents.lines().map(|val| val.to_string()).collect()
}

use std::fs;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
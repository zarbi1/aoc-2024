use std::fs::File;
use std::io::{BufRead, BufReader, Error};
pub fn read_input(file_path: &str) -> Result<Vec<Vec<String>>, Error> {
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        //pushing each line into my vector
        lines.push(
            line?
                .as_str()
                .split_whitespace()
                .map(String::from)
                .collect(),
        );
    }

    Ok(lines)
}

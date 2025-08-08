use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn read_char_matrix<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line_result| line_result.map(|line| line.chars().collect::<Vec<char>>()))
        .collect()
}

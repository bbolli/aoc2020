use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| !l.is_empty())
        .collect()
}

#[allow(dead_code)]
fn lines_as_numbers(lines: Vec<String>) -> Vec<i32> {
    lines.iter().filter_map(|l| l.parse::<i32>().ok()).collect()
}

#[allow(dead_code)]
pub fn numbers_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    lines_as_numbers(lines_from_file(filename))
}

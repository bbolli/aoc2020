use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn lines_as_numbers(lines: Vec<String>) -> Vec<i32> {
    lines.iter().filter_map(|l| l.parse::<i32>().ok()).collect()
}

// ---

fn main() {
    let lines = lines_from_file("01.in");
    let numbers = lines_as_numbers(lines);

    'outer1: for a in &numbers {
        for b in &numbers {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'outer1;
            }
        }
    }

    'outer2: for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'outer2;
                }
            }
        }
    }
}

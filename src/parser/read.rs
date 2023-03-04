use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line() {}

pub fn read_file() {
    let file = File::open("NP.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}

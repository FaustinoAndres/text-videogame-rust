use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

fn main() {

    let content = fs::read_to_string(FILENAME).unwrap();
    println!("{}", content);
}

use csv::{ReaderBuilder, StringRecord};
use std::{fs};

const FILENAME: &str = "history.csv";

fn main() {

    let content = fs::read_to_string(FILENAME).unwrap();
    println!("{}", content);

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {

        println!("texto: {}", result.unwrap().get(2).unwrap().trim());

    }
}

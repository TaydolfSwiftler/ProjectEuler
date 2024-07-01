use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn txt_file_parser() -> Vec<String> {
    let file = File::open(
        "C:\\Users\\Max\\RustroverProjects\\ProjectEuler\\learn_rust\\Files\\0059_cipher.txt",
    )
    .unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// TODO: Finish

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: &File) {
    let bf = BufReader::new(input);

    for _l in bf.lines() {}
}

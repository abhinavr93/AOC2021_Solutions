use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: &File) {
    let bf = BufReader::new(input);

    let mut dist: i32 = 0;
    let mut depth: i32 = 0;

    for l in bf.lines() {
        let str = l.unwrap();
        let mut iter_split = str.split(' ');

        let s = iter_split.next().unwrap();
        let n = iter_split.next().unwrap().parse::<i32>().unwrap();

        if s == "forward" {
            dist += n;
        }
        if s == "down" {
            depth += n;
        }
        if s == "up" {
            depth -= n;
        }
    }

    println!("Horizontal Distance: {} ", dist);
    println!("Depth: {}", depth);
    println!("Product: {}", dist * depth);
}

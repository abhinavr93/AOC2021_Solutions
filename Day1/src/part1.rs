use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: &File) {
    let val = BufReader::new(input)
        .lines()
        .fold((0, -1), |(num, prev), x| -> (i32, i32) {
            let i = x.unwrap().parse::<i32>().unwrap();
            if i > prev && prev != -1 {
                return (num + 1, i);
            } else {
                return (num, i);
            }
        });

    println!("Number of increments: {}", val.0);

}

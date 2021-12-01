use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn solve(input: &File) {
    let val = BufReader::new(input)
        .lines().map(|x| x.unwrap().parse::<i32>().unwrap()).tuple_windows::<(_,_,_)>()
        .fold((0, -1), |(num, prev), x| -> (i32, i32) {
            let cur = x.0 + x.1 + x.2;
            if cur > prev && prev != -1 {
                return (num + 1, cur);
            } else {
                return (num, cur);
            }
        });

    println!("Number of increments of sums: {}", val.0);
}
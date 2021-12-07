use std::fs::File;
use std::io::Read;

pub fn solve(input: &mut File) {
    let mut buf = String::new();
    input.read_to_string(&mut buf);

    let mut arr = buf
        .split(',')
        .filter_map(|c| c.parse::<i32>().ok())
        .collect::<Vec<_>>();

    arr.sort_unstable();
    let len = arr.len();
    let median = if len % 2 == 0 {
        (arr[len / 2] + arr[len / 2 - 1]) / 2
    } else {
        arr[len / 2]
    };

    let fuel_cost = arr.iter().fold(0, |acc, &a| acc + (median - a).abs());

    println!("Median: {}, Fuel Cost: {}", median, fuel_cost);
}

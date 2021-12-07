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
    let len = arr.len() as f32;
    let mean = arr.iter().sum::<i32>() as f32 / len;
    let (mean1, mean2) = (mean.floor() as i32, mean.ceil() as i32);

    let cost1 = fuel_cost(mean1, &arr);
    let cost2 = fuel_cost(mean2, &arr);

    let result = if cost1 < cost2 {
        (mean1, cost1)
    } else {
        (mean2, cost2)
    };
    println!("Mean: {}, Fuel Cost: {}", result.0, result.1);
}

fn fuel_cost(m: i32, arr: &Vec<i32>) -> i32 {
    arr.iter().fold(0, |acc, &a| {
        let n = (m - a).abs();
        acc + n * (n + 1) / 2
    })
}

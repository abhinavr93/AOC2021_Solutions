use std::fs::File;
use std::io::{BufRead, BufReader};

const N: usize = 1000;

pub fn solve(input: &File) {
    let bf = BufReader::new(input);

    let mut arr = vec![vec![0_u32; N]; N];
    let mut final_count = 0_u32;

    for l in bf.lines() {
        if !l.as_ref().unwrap().is_empty() {
            let coord = l
                .unwrap()
                .split(|c| c == ',' || c == ' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            if coord[0] == coord[2] {
                let x = coord[0];

                let yrange = if coord[1] <= coord[3] {
                    coord[1]..=coord[3]
                } else {
                    coord[3]..=coord[1]
                };

                for y in yrange {
                    arr[y][x] += 1;

                    if arr[y][x] == 2 {
                        final_count += 1;
                    }
                }
            } else if coord[1] == coord[3] {
                let y = coord[1];

                let xrange = if coord[0] <= coord[2] {
                    coord[0]..=coord[2]
                } else {
                    coord[2]..=coord[0]
                };

                for x in xrange {
                    arr[y][x] += 1;
                    if arr[y][x] == 2 {
                        final_count += 1;
                    }
                }
            }
        }
    }

    println!("Final Count: {}", final_count);
}

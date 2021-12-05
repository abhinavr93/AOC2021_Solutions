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
            } else {
                let x_increment = if coord[0] <= coord[2] { 1 } else { -1 };

                let y_increment = if coord[1] <= coord[3] { 1 } else { -1 };

                let mut x = coord[0];
                let mut y = coord[1];
                while !(x == incr(coord[2], x_increment) && y == incr(coord[3], y_increment)) {
                    arr[y][x] += 1;
                    if arr[y][x] == 2 {
                        final_count += 1;
                    }
                    x = incr(x, x_increment);
                    y = incr(y, y_increment);
                }
            }
        }
    }

    //println!("{:?}", arr);
    println!("Final Count: {}", final_count);
}

fn incr(v: usize, i: i32) -> usize {
    (v as i32 + i) as usize
}
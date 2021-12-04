use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input: &File) {
    let bf = BufReader::new(input);

    let mut count: Vec<u32> = Vec::new();
    let mut lno: u32 = 0;
    for l in bf.lines() {
        for (i, n) in l
            .unwrap()
            .chars()
            .map(|c| c.to_digit(2).unwrap())
            .enumerate()
        {
            match lno {
                0 => {
                    if n == 1 {
                        count.push(1)
                    } else {
                        count.push(0)
                    }
                }
                _ => {
                    if n == 1 {
                        count[i] += 1
                    }
                }
            }
        }

        lno += 1;
    }

    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    let size: u32 = count.len() as u32;

    for (i, &n) in count.iter().enumerate() {
        if n > lno / 2 {
            num1 += 2_u32.pow(size - i as u32 - 1);
        } else {
            num2 += 2_u32.pow(size - i as u32 - 1);
        }
    }

    println!("------PART1 SOLUTION------\nProduct: {}", num1 * num2);
}

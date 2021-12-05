use std::fs::File;
use std::io::{BufRead, BufReader};

/*
The input file needed an extra line to make sure the last board is being processed.
 */
pub fn solve(input: &File) {
    let bf = BufReader::new(input);

    let mut line_iter = bf.lines();

    let num_arr = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut board: Vec<Vec<(u32, bool)>> = vec![vec![(0, false); 5]; 5];
    let mut row = 0;

    let mut min_bingo_idx = num_arr.len();
    let mut bingo_board: Vec<Vec<(u32, bool)>> = vec![vec![(0, false); 5]; 5];
    //println!("{:?}", board);
    line_iter.next();
    for line in line_iter {
        //println!("{:?}", line.as_ref().unwrap());
        if line.as_ref().unwrap() == "" {
            row = 0;
            let mut row_bingo: Vec<u32> = vec![0; 5];
            let mut col_bingo: Vec<u32> = vec![0; 5];
            'num_loop: for idx in 0..num_arr.len() {
                for i in 0..5 {
                    for j in 0..5 {
                        if board[i][j].0 == num_arr[idx] {
                            board[i][j].1 = true;
                            row_bingo[i] += 1;
                            col_bingo[j] += 1;

                            if row_bingo[i] == 5 || col_bingo[j] == 5 {
                                if idx < min_bingo_idx {
                                    min_bingo_idx = idx;
                                    bingo_board = board.clone();
                                }
                                break 'num_loop;
                            }
                        }
                    }
                }
            }
        } else {
            board[row] = line
                .unwrap()
                .split_whitespace()
                .map(|c| (c.parse::<u32>().unwrap(), false))
                .collect();
            row += 1;
        }
    }

    let sum = bingo_board.iter().fold(0, |sum, v| {
        sum + v
            .iter()
            .filter(|&&e| e.1 == false)
            .fold(0, |rsum, &e| rsum + e.0)
    });

    println!("Final Answer for bingo board winning first: {}", sum * num_arr[min_bingo_idx]);
}

use std::fs::File;
use std::io::Read;

const N: usize = 80;

pub fn solve(input: &mut File) {
    let mut buf = String::new();
    input.read_to_string(&mut buf);

    let mut arr = vec![(0_u32, 0_u32); 7];

    buf.split(',')
        .filter_map(|c| c.parse::<usize>().ok())
        .for_each(|t0| arr[t0].0 += 1);

    println!("{:?}", arr);

    for i in 1..=N {
        let idx1 = (i + 1) % 7;
        let idx2 = (i - 1) % 7;
        arr[idx1].1 = arr[idx2].0;
        arr[idx2].0 += arr[idx2].1;
        arr[idx2].1 = 0;
    }

    let sum = arr.iter().fold(0, |sum, x| sum + x.0 + x.1);

    println!("Final Number of Fishes: {}", sum);
}

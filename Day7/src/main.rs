use std::fs::File;

mod part1;
mod part2;

fn main() {
    let mut input = File::open("input/input.txt").unwrap();

    part1::solve(&mut input);

    let mut input = File::open("input/input.txt").unwrap();

    part2::solve(&mut input);
}

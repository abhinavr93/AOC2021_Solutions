use std::fs::File;

mod part1;
mod part2;

fn main() {
    let input = File::open("input/input.txt").unwrap();

    part1::solve(&input);

    let input = File::open("input/input.txt").unwrap();
    
    part2::solve(&input);
}

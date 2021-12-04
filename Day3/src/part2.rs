use std::{fs::File, io::Read};

pub fn solve(input: &mut File) {
    let mut buf = String::new();
    let _s = input.read_to_string(&mut buf);

    let arr: Vec<Vec<u32>> = buf
        .split('\n')
        .map(|s| s.trim().chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();

    let len = arr[0].len();

    let mut arr_most_common: Vec<Vec<u32>>;
    let mut arr_least_common: Vec<Vec<u32>>;

    let (arr0, arr1) = get_subarrays(0, arr);

    if arr1.len() >= arr0.len() {
        arr_most_common = arr1;
        arr_least_common = arr0;
    } else {
        arr_most_common = arr0;
        arr_least_common = arr1;
    }

    for i in 1..len {
        if arr_most_common.len() != 1 {
            let (a0, a1) = get_subarrays(i, arr_most_common);
            if a1.len() >= a0.len() {
                arr_most_common = a1;
            } else {
                arr_most_common = a0;
            }
        }

        if arr_least_common.len() != 1 {
            let (a0, a1) = get_subarrays(i, arr_least_common);
            if a1.len() >= a0.len() {
                arr_least_common = a0;
            } else {
                arr_least_common = a1;
            }
        }
    }

    println!(
        "\n-------PART2 SOLUTION-------\narr_most_common: {:?}\narr_least_common: {:?}",
        arr_most_common, arr_least_common
    );

    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    let blen = arr_most_common[0].len();
    for i in (0..blen).rev() {
        num1 += arr_most_common[0][blen - i - 1] * 2_u32.pow(i as u32);
        num2 += arr_least_common[0][blen - i - 1] * 2_u32.pow(i as u32);
    }

    println!("Product: {}", num1 * num2);
}

fn get_subarrays(bit_pos: usize, arr: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    //let mut sub_arr: Vec<Vec<u32>> = Vec::new();
    arr.iter()
        .fold((Vec::new(), Vec::new()), |(arr0, arr1), bin| {
            let mut ret_arr0 = arr0;
            let mut ret_arr1 = arr1;
            match bin[bit_pos] {
                0 => ret_arr0.push(bin.clone()),
                1 => ret_arr1.push(bin.clone()),
                _ => (),
            }
            (ret_arr0, ret_arr1)
        })
}

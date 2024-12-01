use std::io::Error;

use aoc::aoc_common;

// change result type to Result<i64, Error> to return a number
fn solve(input: String) -> Result<String, Error> {
    let lines = aoc_common::to_lines(input);
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in lines {
        let (left_val, right_val) = line.split_once("   ").expect("Invalid row");
        left.push(left_val.parse::<i64>().unwrap());
        right.push(right_val.parse::<i64>().unwrap());
    }
    // sort left and right
    left.sort();
    right.sort();
    let mut diff_sum = 0;
    for i in 0..left.len() {
        let diff = left[i].abs_diff(right[i]);
        diff_sum += diff;
    }
    // return the sum of the differences
    Ok(diff_sum.to_string())
}

fn main() {
    // println! is a macro that prints to the console
    // get argv[1] and pass it to solve
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
}

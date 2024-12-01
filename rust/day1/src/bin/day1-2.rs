use std::{
    collections::HashMap,
    io::{self, Error},
};

use aoc::aoc_common;

fn solve(input: String) -> Result<String, Error> {
    let lines = aoc_common::to_lines(input);

    let mut left_values_arr: Vec<u32> = Vec::new();
    let mut rightside_num_occurences_map: HashMap<u32, u32> = HashMap::new();
    let mut total: u64 = 0;
    // let mut right_values_arr: Vec<u32> = Vec::new();

    for line in lines {
        let (left_str, right_str) = line.split_once("   ").expect("Invalid row");
        let left = left_str.parse::<u32>().unwrap();
        let right = right_str.parse::<u32>().unwrap();

        left_values_arr.push(left);
        // right_values_arr.push(right);
        rightside_num_occurences_map
            .entry(right)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    for left_val in left_values_arr {
        total += (left_val * rightside_num_occurences_map.get(&left_val).unwrap_or(&0)) as u64;
    }

    Ok(total.to_string())
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {}", solution);
    Ok(())
}

use aoc::solve_input_file;
use std::io::Error;

fn solve(input: String) -> Result<String, Error> {
    let result: u64 = input.lines().fold(0u64, |mut acc, line| {
        let (left, right) = line.split_once(": ").unwrap();
        let left_num: u64 = left.parse::<u64>().unwrap();
        let nums = right
            .split_ascii_whitespace()
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let num_ops = nums.len() - 1;
        let num_combis = u32::pow(2, num_ops as u32); // number of combinations = 2 ^ number_of_operators
        'combi_tries: for combi in 0..num_combis {
            let combi_binary = format!("{:0>num_ops$}", format!("{:0b}", combi), num_ops = num_ops); // binary string of 0s and 1s
            let combi_product: u64 =
                combi_binary
                    .chars()
                    .enumerate()
                    .fold(nums[0], |acc, (bin_idx, bin_type)| match bin_type {
                        '1' => acc * nums[bin_idx + 1],
                        '0' => acc + nums[bin_idx + 1],
                        _ => panic!("inv bin type"),
                    });
            if combi_product == left_num {
                println!("{}: {} is okay", left_num, right);
                acc += combi_product as u64;
                break 'combi_tries;
            }
        }
        acc
    });
    Ok(format!("{}", result))
}

fn main() {
    solve_input_file(solve)
}

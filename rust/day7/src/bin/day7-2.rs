use aoc::solve_input_file;
use radix_fmt::radix_3;
use rayon::{iter::ParallelIterator, str::ParallelString};
use std::io::Error;

fn solve(input: String) -> Result<String, Error> {
    let result: u64 = input
        .par_lines()
        .fold_with(0u64, |mut acc, line| {
            let (left, right) = line.split_once(": ").unwrap();
            let left_num: u64 = left.parse::<u64>().unwrap();
            let nums = right
                .split_ascii_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let num_ops = nums.len() - 1; // e.g. if 3 numbers, num_ops = 3 - 1 = 2
            let num_combis = u32::pow(3, num_ops as u32); // number of combinations = 3 POW num_operations (3 to-the-power-of number_of_operations)

            'combi_tries: for combi in 0..num_combis {
                let radix3_trinary = radix_3(combi).to_string();
                // println!("> trying {} for {}: {}", combi_binary, left_num, right);
                let combi_binary = format!("{:0>num_ops$}", radix3_trinary, num_ops = num_ops); // binary string of 0s and 1s
                let combi_product: u64 =
                    combi_binary
                        .chars()
                        .enumerate()
                        .fold(nums[0], |acc, (bin_idx, bin_type)| match bin_type {
                            '2' => {
                                // JavaScript:
                                // return parseInt(`${acc}${val}`)
                                let num: u64 = nums[bin_idx + 1];
                                let str = acc.to_string() + &num.to_string();
                                str.parse::<u64>().unwrap()
                            }
                            // return acc x val
                            '1' => acc * nums[bin_idx + 1],
                            // return acc + val
                            '0' => acc + nums[bin_idx + 1],
                            _ => panic!("inv bin type"),
                        });
                if combi_product == left_num {
                    // println!("{}: {} is okay", left_num, right);
                    acc += combi_product as u64;
                    break 'combi_tries;
                } else {
                    // println!("  -> {}", combi_product);
                }
            }
            acc
        })
        .sum();
    Ok(format!("{}", result))
}

fn main() {
    solve_input_file(solve)
}

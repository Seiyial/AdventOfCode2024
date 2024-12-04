use aoc::solve_input_file_to_int;
use regex::Regex;

fn solve(input: String) -> u64 {
    Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don\'t\(\))")
        .unwrap()
        .captures_iter(input.as_str())
        .fold((true, 0), |(is_active, sum), inst| {
            match inst.get(0).unwrap().as_str() {
                "don't()" => (false, 0),
                "do()" => (true, 0),
                mul_str if is_active => (
                    true,
                    mul_str[4..(mul_str.len() - 1)]
                        .split(",")
                        .map(|a| a.parse::<u64>().unwrap())
                        .product(),
                ),
                _ => (is_active, sum),
            }
        })
        .1
}

fn main() {
    solve_input_file_to_int(solve)
}

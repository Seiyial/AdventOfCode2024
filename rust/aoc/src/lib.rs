use std::io::Error;

pub mod aoc_common;

pub fn read_argv_input_file() -> String {
    let input_path = std::env::args().nth(1).expect("No input_path given");
    std::fs::read_to_string(input_path).expect("Failed to read input")
}

pub fn solve_input_file_to_int(solve: fn(input: String) -> u64) {
    let input = read_argv_input_file();
    println!("Solution: {:?}", solve(input));
}
pub fn solve_input_file(solve: fn(input: String) -> Result<String, Error>) {
    let input = read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");
    println!("Solution: {}", solution);
}

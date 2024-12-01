use day3::{debugger, model::Next};
use std::{
    io::{self, Error},
    ops::RangeInclusive,
};

const VALID_NUM1: RangeInclusive<char> = RangeInclusive::new('1', '9');
const VALID_NUM23: RangeInclusive<char> = RangeInclusive::new('0', '9');

fn solve(input: String) -> Result<u64, Error> {
    let characters: Vec<char> = input.replace("\n", "").chars().collect();
    let mut sum: u64 = 0;
    for start_index in 0..characters.len() {
        let mut next: Next = Next::M;
        'inner: for end_index in 0 as usize.. {
            let char = characters.get(start_index + end_index);
            if char.is_none() {
                break 'inner;
            };
            let char = char.unwrap();
            debugger::print_vals(char, &next);
            let mut cleared = false;
            match next {
                Next::M if *char == 'm' => next = Next::U,
                Next::U if *char == 'u' => next = Next::L,
                Next::L if *char == 'l' => next = Next::Open,
                Next::Open if *char == '(' => next = Next::Num1_1,
                Next::Num1_1 if VALID_NUM1.contains(char) => next = Next::Num1_2,
                Next::Num1_2 if VALID_NUM23.contains(char) => next = Next::Num1_3,
                Next::Num1_2 if *char == ',' => next = Next::Num2_1,
                Next::Num1_3 if VALID_NUM23.contains(char) => next = Next::Comma,
                Next::Num1_3 if *char == ',' => next = Next::Num2_1,
                Next::Comma if *char == ',' => next = Next::Num2_1,
                Next::Num2_1 if VALID_NUM1.contains(char) => next = Next::Num2_2,
                Next::Num2_2 if VALID_NUM23.contains(char) => next = Next::Num2_3,
                Next::Num2_2 if *char == ')' => cleared = true,
                Next::Num2_3 if VALID_NUM23.contains(char) => next = Next::Close,
                Next::Num2_3 if *char == ')' => cleared = true,
                Next::Close if *char == ')' => cleared = true,
                _ => {
                    if next != Next::M {
                        println!("^ not ok, {} {:?}", VALID_NUM1.contains(char), VALID_NUM1);
                    };
                    break 'inner;
                }
            };
            if cleared {
                let use_start_index = start_index + 4; // mul(
                let use_end_index = start_index + end_index - 1; // )
                let slice = (&characters[use_start_index..=use_end_index])
                    .iter()
                    .collect::<String>();
                println!("{}", slice);
                let strs = slice.split_once(",").expect("parsed wrong item");
                let value: u64 = strs.0.parse::<u64>().expect("item1 not num")
                    * strs.1.parse::<u64>().expect("item2 not num");
                sum += value;
                break 'inner;
            }
        }
    }
    Ok(sum)
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
    Ok(())
}

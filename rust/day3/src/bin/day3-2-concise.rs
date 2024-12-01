use std::{
    io::{self, Error},
    ops::RangeInclusive,
};

use day3::model::Next;

const VALID_NUM1: RangeInclusive<char> = RangeInclusive::new('1', '9');
const VALID_NUM23: RangeInclusive<char> = RangeInclusive::new('0', '9');
const DO_SEQ: [char; 4] = ['d', 'o', '(', ')'];
const DONT_SEQ: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];

fn solve(input: String) -> Result<u64, Error> {
    let chars: Vec<char> = input.replace("\n", "").chars().collect();
    let mut sum: u64 = 0;
    let mut is_crawling = true;
    for start_index in 0..chars.len() {
        if let false = match chars[start_index] {
            'd' => {
                let i = start_index;
                match is_crawling {
                    // this may overaccess chars but fixable
                    false if chars[i..i + DO_SEQ.len()] == DO_SEQ => is_crawling = true,
                    true if chars[i..i + DONT_SEQ.len()] == DONT_SEQ => is_crawling = false,
                    _ => (),
                }
                false
            }
            'm' => is_crawling,
            _ => false,
        } {
            continue;
        }

        let mut next: Next = Next::M;
        'inner: for end_index in 0 as usize.. {
            let char = match chars.get(start_index + end_index) {
                None => break 'inner,
                Some(val) => val,
            };
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
                _ => break 'inner,
            };
            if cleared {
                let slice = (&chars[(start_index + 4)..=(start_index + end_index - 1)])
                    .iter()
                    .collect::<String>();
                let strs = slice.split_once(",").unwrap();
                sum += strs.0.parse::<u64>().unwrap() * strs.1.parse::<u64>().unwrap();
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

use std::fmt::Debug;
use std::str::FromStr;

pub fn to_lines(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn to_2d_array<ParseIntoType: FromStr>(input: String) -> Vec<Vec<ParseIntoType>>
where
    ParseIntoType: FromStr<Err: Debug>,
{
    input
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|cell| {
                    cell.parse::<ParseIntoType>()
                        .expect("parsed type not supported by a value")
                })
                .collect::<Vec<ParseIntoType>>()
        })
        .collect::<Vec<Vec<ParseIntoType>>>()
}

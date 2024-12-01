use std::io::{self, Error};

fn solve(input: String) -> Result<i32, Error> {
    let aoa: Vec<Vec<u32>> = input
        .split("\n")
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let ans = aoa
        .clone()
        .into_iter()
        .enumerate()
        .fold(0, |acc, (line_index, line)| {
            let mut delta_type = 0;
            for (cell_index, report) in line.into_iter().enumerate() {
                if cell_index == 0 {
                    continue;
                } else {
                    let delta = report as i32 - aoa[line_index][cell_index - 1] as i32;
                    let delta_abs = delta.abs();
                    if delta_abs > 3 || delta_abs < 1 {
                        return acc;
                    };
                    if delta_type == 0 {
                        delta_type = delta;
                    } else if delta_type.is_positive() != delta.is_positive() {
                        return acc;
                    } else {
                        continue;
                    }
                }
            }
            println!("+1 at row {}", line_index);
            acc + 1
        });

    Ok(ans)
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
    Ok(())
}

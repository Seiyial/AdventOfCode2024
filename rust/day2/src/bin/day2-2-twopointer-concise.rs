use std::io::{self, Error};

use aoc::aoc_common;

fn get_delta(aoa: &Vec<Vec<u32>>, line_index: usize, last_used_index: i32, current: u32) -> i32 {
    let last = aoa[line_index][last_used_index as usize];
    let delta = current as i32 - last as i32;
    delta
}

fn is_bounded(delta: i32) -> bool {
    delta != 0 && delta.abs() <= 3
}

fn is_safe(delta: i32, delta_type: i32) -> bool {
    is_bounded(delta) && ((delta > 0 && delta_type > 0) || (delta < 0 && delta_type < 0))
}

fn solve(input: String) -> Result<i32, Error> {
    let aoa: Vec<Vec<u32>> = aoc_common::to_2d_array::<u32>(input);
    let ans = aoa
        .clone()
        .into_iter()
        .enumerate()
        .fold(0, |acc, (line_index, line)| {
            let any_attempt_pass = (-1..(line.len() as i32)).any(|attempt_idx_and_omit_idx| {
                let mut delta_type = 0;
                let mut last_used_index: i32 = -1;
                (0..line.len()).all(|cell_index| {
                    let current = line[cell_index];
                    if attempt_idx_and_omit_idx == -1 {
                        match cell_index {
                            0 => {
                                last_used_index = 0;
                                true
                            }
                            1 => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = cell_index as i32;
                                delta_type = delta.signum();
                                is_bounded(delta)
                            }
                            _rest => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = cell_index as i32;
                                is_safe(delta, delta_type)
                            }
                        }
                    } else if attempt_idx_and_omit_idx == 0 {
                        match cell_index {
                            0 => true,
                            1 => {
                                last_used_index = 1;
                                true
                            }
                            2 => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = 2 as i32;
                                delta_type = delta.signum();
                                is_bounded(delta)
                            }
                            _rest => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = cell_index as i32;
                                is_safe(delta, delta_type)
                            }
                        }
                    } else {
                        match cell_index {
                            0 => {
                                last_used_index = 0;
                                true
                            }
                            index if index as i32 == attempt_idx_and_omit_idx => true,
                            _index if delta_type == 0 => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = cell_index as i32;
                                delta_type = delta.signum();
                                is_bounded(delta)
                            }
                            _rest => {
                                let delta = get_delta(&aoa, line_index, last_used_index, current);
                                last_used_index = cell_index as i32;
                                is_safe(delta, delta_type)
                            }
                        }
                    }
                })
            });
            return if any_attempt_pass { acc + 1 } else { acc };
        });
    Ok(ans)
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
    Ok(())
}

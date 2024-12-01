use std::{
    collections::HashSet,
    io::{self, Error},
};

use day2::printer;

fn to_2d_array(input: String) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn get_delta_and_print(
    aoa: &Vec<Vec<u32>>,
    line_index: usize,
    last_used_index: usize,
    current: u32,
) -> i32 {
    let last = aoa[line_index][last_used_index];
    let delta = current as i32 - last as i32;
    // println!("{} - {}", current, last);
    printer::print_delta(delta);
    delta
}

fn expect(delta_type: i32) {
    if delta_type == 0 {
        panic!("delta type not set")
    }
}

fn solve(input: String) -> Result<i32, Error> {
    let aoa: Vec<Vec<u32>> = to_2d_array(input);

    let ans = aoa.clone().into_iter().enumerate().fold(
        HashSet::<usize>::new(),
        |mut acc, (line_index, line)| {
            let any_attempt_pass = (-1..(line.len() as i32)).any(|attempt_idx_and_omit_idx| {
                printer::print_attempt(line_index, attempt_idx_and_omit_idx);
                let mut delta_type = 0;

                let mut last_used_index: i32 = -1;
                let attempt_ok = (0..line.len()).all(|cell_index| {
                    let current = line[cell_index];
                    if attempt_idx_and_omit_idx == -1 {
                        if cell_index == 0 {
                            last_used_index = 0;
                            printer::print_first_value(cell_index, current);
                            return true;
                        }

                        if cell_index == 1 {
                            printer::print_value(cell_index, current);
                            let delta = get_delta_and_print(
                                &aoa,
                                line_index,
                                last_used_index as usize,
                                current,
                            );
                            last_used_index = cell_index as i32;
                            if delta == 0 {
                                return false;
                            } else if delta.abs() > 3 {
                                return false;
                            } else if delta > 0 {
                                delta_type = 1
                            } else if delta < 0 {
                                delta_type = -1
                            }
                            return true;
                        }

                        expect(delta_type);

                        printer::print_value(cell_index, current);
                        let delta = get_delta_and_print(
                            &aoa,
                            line_index,
                            last_used_index as usize,
                            current,
                        );
                        last_used_index = cell_index as i32;
                        if delta == 0 {
                            return false;
                        } else if delta.abs() > 3 {
                            return false;
                        } else if delta > 0 {
                            return delta_type > 0;
                        } else if delta < 0 {
                            return delta_type < 0;
                        }
                        panic!("Invalid case scenario")
                    } else if attempt_idx_and_omit_idx == 0 {
                        if cell_index == 0 {
                            printer::print_skipped(cell_index, current);
                            return true;
                        }

                        if cell_index == 1 {
                            last_used_index = cell_index as i32;
                            printer::print_first_value(cell_index, current);
                            return true;
                        }

                        if cell_index == 2 {
                            printer::print_value(cell_index, current);
                            let delta = get_delta_and_print(
                                &aoa,
                                line_index,
                                last_used_index as usize,
                                current,
                            );
                            last_used_index = cell_index as i32;
                            if delta == 0 {
                                return false;
                            } else if delta.abs() > 3 {
                                return false;
                            } else if delta > 0 {
                                delta_type = 1
                            } else if delta < 0 {
                                delta_type = -1
                            }
                            return true;
                        }

                        expect(delta_type);

                        printer::print_value(cell_index, current);
                        let delta = get_delta_and_print(
                            &aoa,
                            line_index,
                            last_used_index as usize,
                            current,
                        );
                        last_used_index = cell_index as i32;
                        if delta == 0 {
                            false
                        } else if delta.abs() > 3 {
                            false
                        } else if delta > 0 {
                            delta_type > 0
                        } else {
                            delta_type < 0
                        }
                    } else {
                        if cell_index == 0 {
                            last_used_index = 0;
                            printer::print_first_value(cell_index, current);
                            return true;
                        }

                        if cell_index as i32 == attempt_idx_and_omit_idx {
                            printer::print_skipped(cell_index, current);
                            return true;
                        }

                        if delta_type == 0 {
                            printer::print_value_first_delta(cell_index, current);
                            let delta = get_delta_and_print(
                                &aoa,
                                line_index,
                                last_used_index as usize,
                                current,
                            );
                            last_used_index = cell_index as i32;
                            if delta == 0 {
                                return false;
                            } else if delta.abs() > 3 {
                                return false;
                            } else if delta > 0 {
                                delta_type = 1
                            } else if delta < 0 {
                                delta_type = -1
                            }
                            return true;
                        }

                        expect(delta_type);

                        printer::print_value(cell_index, current);
                        let delta = get_delta_and_print(
                            &aoa,
                            line_index,
                            last_used_index as usize,
                            current,
                        );
                        last_used_index = cell_index as i32;
                        if delta == 0 || delta.abs() > 3 {
                            false
                        } else if delta > 0 {
                            delta_type > 0
                        } else {
                            delta_type < 0
                        }
                    }
                });

                return attempt_ok;
            });
            if any_attempt_pass {
                println!("\n==> Row {} Safe", line_index + 1);
                acc.insert(line_index);
            } else {
                println!("\n==> Row {} Unsafe", line_index + 1);
            }
            acc
        },
    );

    println!("{:?}", ans);

    Ok(ans.len() as i32)
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
    Ok(())
}

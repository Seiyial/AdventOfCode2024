use std::{
    collections::HashSet,
    io::{self, Error},
};

fn solve(input: String) -> Result<i32, Error> {
    let aoa: Vec<Vec<u32>> = input
        .split("\n")
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let ans = aoa.clone().into_iter().enumerate().fold(
        HashSet::<usize>::new(),
        |mut acc, (line_index, line)| {
            let mut delta_type: i32 = 0;
            let any_attempt_pass = (-1..(line.len() as i32)).any(|attempt_idx_and_omit_idx| {
                println!(
                    "\n=> Row {} {}",
                    line_index + 1,
                    if attempt_idx_and_omit_idx == -1 {
                        "".to_string()
                    } else {
                        format!("omitting #{}", attempt_idx_and_omit_idx + 1)
                    }
                );

                let spliced_iter = line
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _val)| *idx as i32 != attempt_idx_and_omit_idx)
                    .map(|(_i, item)| item);
                let spliced_arr: Vec<u32> = spliced_iter.clone().collect();

                let ok = spliced_iter.enumerate().all(|(cell_index, current)| {
                    println!("#{} = {}", cell_index + 1, current);
                    if cell_index == 0 {
                        true
                    } else if cell_index == 1 {
                        let last = spliced_arr[cell_index - 1] as i32;
                        let delta = current as i32 - last;
                        println!("      ({}{})", if delta > 0 { "+" } else { "" }, delta);
                        if delta == 0 || delta.abs() > 3 {
                            return false;
                        } else if delta > 0 {
                            delta_type = 1;
                        } else if delta < 0 {
                            delta_type = -1;
                        }
                        true
                    } else {
                        let last = spliced_arr[cell_index - 1] as i32;
                        let delta = current as i32 - last;
                        println!("      ({}{})", if delta > 0 { "+" } else { "" }, delta);

                        if delta == 0 || delta.abs() > 3 {
                            println!("  !deltaoutofrange");
                            false
                        } else if delta_type == 0 {
                            panic!("delta_type not set");
                        } else if (delta.is_positive() && delta_type.is_negative())
                            || (delta.is_negative() && delta_type.is_positive())
                        {
                            println!("  !deltadiffdirections");
                            false
                        } else {
                            true
                        }
                    }
                });

                return ok;
            });
            if any_attempt_pass {
                println!("\n==> Row {} Safe", line_index + 1);
                acc.insert(line_index);
            } else {
                println!("\n==> Row {} Unsafe", line_index + 1);
                // acc
            }
            acc
        },
    );

    println!("lines: {:?}", ans);
    Ok(ans.len() as i32)
}

fn main() -> io::Result<()> {
    let input = aoc::read_argv_input_file();
    let solution = solve(input).expect("Failed to solve");

    println!("Solution: {:?}", solution);
    Ok(())
}

pub mod printer {
    pub fn print_attempt(line_index: usize, attempt_idx_and_omit_idx: i32) {
        println!(
            "\n=> Row {} {}",
            line_index + 1,
            if attempt_idx_and_omit_idx == -1 {
                "".to_string()
            } else {
                format!("omitting #{}", attempt_idx_and_omit_idx + 1)
            }
        );
    }

    pub fn print_value(cell_index: usize, current: u32) {
        println!("#{} = {}", cell_index + 1, current)
    }

    pub fn print_value_first_delta(cell_index: usize, current: u32) {
        println!("#{} = {} (first delta)", cell_index + 1, current)
    }

    pub fn print_first_value(cell_index: usize, current: u32) {
        println!("#{} = {} (first)", cell_index + 1, current)
    }

    pub fn print_skipped(cell_index: usize, _current: u32) {
        println!("#{} = (skipped)", cell_index + 1)
    }

    pub fn print_delta(delta: i32) {
        println!("      ({}{})", if delta > 0 { "+" } else { "" }, delta)
    }
}

pub mod utils {
    pub fn into_2d_array(input: String) -> Vec<Vec<u32>> {
        input
            .split("\n")
            .map(|row| {
                row.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
    }
}

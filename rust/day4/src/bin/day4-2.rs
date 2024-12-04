use aoc::solve_input_file_to_int;

fn is_valid_diag(diag: String) -> bool {
    diag == "MAS" || diag == "SAM"
}
fn has_cross(lines: &Vec<Vec<char>>, x: i32, y: i32, x_max: i32, y_max: i32) -> bool {
    0 < x
        && x < x_max
        && 0 < y
        && y < y_max
        && is_valid_diag(format!(
            "{}A{}",
            lines[(y - 1) as usize][(x - 1) as usize],
            lines[(y + 1) as usize][(x + 1) as usize]
        ))
        && is_valid_diag(format!(
            "{}A{}",
            lines[(y + 1) as usize][(x - 1) as usize],
            lines[(y - 1) as usize][(x + 1) as usize]
        ))
}

fn solve(input: String) -> u64 {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (x_max, y_max) = (
        ((lines.first().unwrap().len() - 1) as i32),
        (lines.len() - 1) as i32,
    );
    lines
        .clone()
        .iter()
        .enumerate()
        .fold(0u64, |spelt, (y0, row_str)| {
            row_str
                .iter()
                .enumerate()
                .fold(spelt, |spelt, (x0, val)| match val {
                    'A' => spelt + has_cross(&lines, x0 as i32, y0 as i32, x_max, y_max) as u64,
                    _ => spelt,
                })
        })
}

fn main() {
    solve_input_file_to_int(solve)
}

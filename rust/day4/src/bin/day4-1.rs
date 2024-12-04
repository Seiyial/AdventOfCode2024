use aoc::solve_input_file_to_int;

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

fn count_spellouts_in(lines: &Vec<Vec<char>>, x0: usize, y0: usize) -> u32 {
    DIRS.clone()
        .iter()
        .map(|(dx, dy)| {
            (1..=3).all(|k| {
                let y: i32 = y0 as i32 + (dy * k);
                let x: i32 = x0 as i32 + (dx * k);
                y >= 0
                    && x >= 0
                    && lines.get(y as usize).is_some_and(|row| {
                        row.get(x as usize)
                            .is_some_and(|char| *char == CHARS[k as usize])
                    })
            }) as u32 // convert true/false to 1/0
        })
        .sum()
}

fn solve(input: String) -> u64 {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    lines
        .clone()
        .iter()
        .enumerate()
        .fold(0u64, |spelt, (y0, row_str)| {
            row_str
                .iter()
                .enumerate()
                .fold(spelt, |spelt, (x0, val)| match val {
                    'X' => spelt + count_spellouts_in(&lines, x0, y0) as u64,
                    _ => spelt,
                })
        })
}

fn main() {
    solve_input_file_to_int(solve)
}

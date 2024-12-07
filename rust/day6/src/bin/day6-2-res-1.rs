// https://www.reddit.com/r/adventofcode/comments/1h7tovg/comment/m0tohev/
// 1.09s

use aoc::solve_input_file;
use rayon::prelude::*;
use std::io::Error;

#[derive(Clone, PartialEq)]
enum Cell {
    Wall,
    Unvisited,
    Visited,
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn simulate(board: &mut [Cell], mut pos: usize) -> bool {
    let size = (board.len() as f64).sqrt() as usize;

    let mut dir = Dir::Up;
    let mut prev = Vec::new();

    loop {
        board[pos] = Cell::Visited;

        if prev.contains(&(pos, dir)) {
            return true;
        }

        let ahead = match dir {
            Dir::Up if pos / size != 0 => pos - size,
            Dir::Right if pos % size != size - 1 => pos + 1,
            Dir::Down if pos / size != size - 1 => pos + size,
            Dir::Left if pos % size != 0 => pos - 1,
            _ => return false,
        };

        if board[ahead] == Cell::Wall {
            prev.push((pos, dir));

            dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        } else {
            pos = ahead;
        }
    }
}

fn main() {
    solve_input_file(solve)
}

fn solve(input: String) -> Result<String, Error> {
    let input = input.replace("\n", "");

    let starting = input.find("^").unwrap();
    let mut board = vec![Cell::Unvisited; input.len()];
    let size = (input.len() as f64).sqrt() as usize;

    for y in 0..size {
        for x in 0..size {
            let idx = y * size + x;
            if input.as_bytes()[idx] == b'#' {
                board[idx] = Cell::Wall;
            }
        }
    }

    let mut part1_board = board.clone();
    assert!(!simulate(&mut part1_board, starting));

    let part1 = part1_board
        .iter()
        .fold(0, |acc, cell| acc + (*cell == Cell::Visited) as usize);

    println!("{}", part1);

    let indices = part1_board
        .iter()
        .enumerate()
        .filter(|(idx, cell)| **cell == Cell::Visited && *idx != starting)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    let part2: usize = indices
        .par_iter()
        .map(|idx| {
            let mut test = board.clone();
            test[*idx] = Cell::Wall;
            simulate(&mut test, starting) as usize
        })
        .sum();

    println!("{}", part2);
    return Ok(part2.to_string());
}

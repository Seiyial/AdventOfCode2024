// https://www.reddit.com/r/adventofcode/comments/1h7tovg/comment/m0tp7ws/
// 0.5s, fastest on our machine on our data

use aoc::solve_input_file;
use std::collections::HashSet;
use std::io::Error;

pub fn part1(input: String) -> String {
    let res = path_set(input).len();
    res.to_string()
}
fn path_set(input: String) -> HashSet<(i32, i32)> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let w = grid[0].len();
    let h = grid.len();
    let mut start = (0, 0);
    let mut rows = vec![vec![]; h];
    let mut cols = vec![vec![]; w];
    for i in 0..w {
        for j in 0..h {
            if grid[j as usize][i as usize] == '^' {
                start = (i as i32, j as i32);
            } else if grid[j as usize][i as usize] == '#' {
                cols[i].push(j as i32);
                rows[j].push(i as i32);
            }
        }
    }
    let mut set = HashSet::new();
    let mut cur = start;
    let mut dir = (0, -1);
    loop {
        let (nx, ny, ends) = get_next(&rows, &cols, cur, dir);
        let mut c = cur;
        let next = (nx, ny);
        while c != next {
            set.insert(c);
            c = (c.0 + dir.0, c.1 + dir.1);
        }
        set.insert(next);
        if ends {
            break;
        }
        cur = next;
        dir = (-dir.1, dir.0);
    }
    return set;
}
fn get_next(
    rows: &Vec<Vec<i32>>,
    cols: &Vec<Vec<i32>>,
    cur: (i32, i32),
    dir: (i32, i32),
    // returns x, y, is_exited
) -> (i32, i32, bool) {
    //rows
    if dir.0 != 0 {
        // if there are no obstacles at where I am
        if let Err(point) = rows[cur.1 as usize].binary_search(&cur.0) {
            // if I'm going left
            if dir.0 == -1 {
                if point == 0 {
                    return (0, cur.1, true);
                } else {
                    return (rows[cur.1 as usize][point - 1] + 1, cur.1, false);
                }
            } else {
                if point == rows[cur.1 as usize].len() {
                    return (cols.len() as i32 - 1, cur.1, true);
                } else {
                    return (rows[cur.1 as usize][point] - 1, cur.1, false);
                }
            }
        } else {
            panic!();
        }
    }
    //cols
    else {
        if let Err(point) = cols[cur.0 as usize].binary_search(&cur.1) {
            if dir.1 == -1 {
                if point == 0 {
                    return (cur.0, 0, true);
                } else {
                    return (cur.0, cols[cur.0 as usize][point - 1] + 1, false);
                }
            } else {
                if point == cols[cur.0 as usize].len() {
                    return (cur.0, rows.len() as i32 - 1, true);
                } else {
                    return (cur.0, cols[cur.0 as usize][point] - 1, false);
                }
            }
        } else {
            panic!();
        }
    }
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let w = grid[0].len();
    let h = grid.len();
    let mut start = (0, 0);
    // 2d array of arrays. each inner array represents all the indexes of obstacles in a row.
    let mut rows = vec![vec![]; h];
    let mut cols = vec![vec![]; w];
    for i in 0..w {
        for j in 0..h {
            if grid[j as usize][i as usize] == '^' {
                start = (i as i32, j as i32);
            } else if grid[j as usize][i as usize] == '#' {
                cols[i].push(j as i32);
                rows[j].push(i as i32);
            }
        }
    }
    let path_set = path_set(input);
    for (i, j) in path_set {
        if (i, j) == start {
            continue;
        }
        let mut cur = start;
        let mut dir = (0, -1);
        let mut set = HashSet::new();
        let mut ri = 0;
        let mut ci = 0;

        // if there is no obstacle at (i, j), insert obstacle at i, j accordingly:
        if let Err(rp) = rows[j as usize].binary_search(&i) {
            rows[j as usize].insert(rp, i);
            ri = rp;
        }
        if let Err(cp) = cols[i as usize].binary_search(&j) {
            cols[i as usize].insert(cp, j);
            ci = cp;
        }

        loop {
            if set.contains(&(cur, dir)) {
                res += 1;
                break;
            }

            // visited
            set.insert((cur, dir));
            //rows
            let (nx, ny, ends) = get_next(&rows, &cols, cur, dir);
            cur = (nx, ny);
            if ends {
                break;
            }
            dir = (-dir.1, dir.0);
        }
        rows[j as usize].remove(ri);
        cols[i as usize].remove(ci);
    }
    res.to_string()
}

fn solve(input: String) -> Result<String, Error> {
    Ok(part2(input))
}

fn main() {
    solve_input_file(solve)
}

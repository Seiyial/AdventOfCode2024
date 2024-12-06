use aoc::solve_input_file;
use day6::model_p2::{Coord, State};
use std::io::Error;

fn solve(input: String) -> Result<String, Error> {
    let grid = day6::common::to_2d_char_array(input);
    let mut state = State::new(grid, None);
    let mut num_possible_obstacle_points: u32 = 0;

    for obstruct_x in 0..state.grid.len() {
        for obstruct_y in 0..state.grid.len() {
            let obstruction_pos = Coord::new(obstruct_x, obstruct_y);
            if state.obstacles.contains(&obstruction_pos) {
                continue;
            } else if state.pos == obstruction_pos {
                continue;
            } else {
                // let mut cloned_state = state.clone();
                println!("trying {}", obstruction_pos);
                state.reset_with_new_obstruction_pos(obstruction_pos);
                match traverse_until_out_or_looped(&mut state) {
                    TraverseResult::Loop => num_possible_obstacle_points += 1,
                    TraverseResult::Out => continue,
                }
            }
        }
    }

    Ok(format!("{}", num_possible_obstacle_points))
}

enum TraverseResult {
    Out,
    Loop,
}
fn traverse_until_out_or_looped(state: &mut State) -> TraverseResult {
    while state.is_inside_grid {
        let next_pos: Coord = state.rotate_as_needed_and_get_next_destination();
        match state.move_to_and_return_whether_looping(next_pos) {
            true => return TraverseResult::Loop,
            false => (),
        }
    }
    return TraverseResult::Out;
}

fn main() {
    solve_input_file(solve)
}

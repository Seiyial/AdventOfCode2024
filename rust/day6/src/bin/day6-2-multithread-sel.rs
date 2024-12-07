use aoc::solve_input_file;
use day6::model_p2_v2::{Coord, State};
use rayon::prelude::*;
use std::io::Error;

fn solve(input: String) -> Result<String, Error> {
    let grid = day6::common::to_2d_char_array(input);
    let state = State::new(grid, None);

    let mut try1_state = state.clone();
    traverse_until_out_or_looped(&mut try1_state);

    let possible_obstructions: u32 = (0..=state.grid_limit.x)
        .map(move |x| {
            (0..=state.grid_limit.y)
                .map(move |y| Coord { x, y })
                .collect::<Vec<Coord>>()
        })
        .flatten()
        .filter(|coord| try1_state.visitations.has_coord(coord))
        .par_bridge()
        .map(|obstruction_pos| {
            let mut cloned_state: State = state.clone();
            cloned_state.obstacles.insert(obstruction_pos.clone());
            traverse_until_out_or_looped(&mut cloned_state)
        })
        .sum();

    Ok(format!("{}", possible_obstructions))
}

fn traverse_until_out_or_looped(state: &mut State) -> u32 {
    while state.is_inside_grid {
        let next_pos: Coord = state.rotate_as_needed_and_get_next_destination();
        match state.move_to_and_return_whether_looping(next_pos) {
            true => return 1,
            false => (),
        }
    }
    0
}

fn main() {
    solve_input_file(solve)
}

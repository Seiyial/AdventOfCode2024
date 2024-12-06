use aoc::solve_input_file;
use day6::model_p1::{Coord, State};
use std::io::Error;

fn solve(input: String) -> Result<String, Error> {
    let grid = day6::common::to_2d_char_array(input);

    let mut state: State = State::new(grid, None);
    while state.is_inside_grid {
        let next_pos: Coord = state.rotate_as_needed_and_get_next_destination();
        state.move_to(next_pos);
    }

    Ok(format!(
        "\nVisited {}\nObstacles {:?}\nExited from {:?} {:?}",
        state.visited.len(),
        state.obstacles.len(), // remove the .len() to see everything
        state.pos,
        state.dir
    ))
}

fn main() {
    solve_input_file(solve)
}

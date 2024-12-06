use std::{collections::HashSet, fmt::Display};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x as i32,
            y: y as i32,
        }
    }
    /// if None, it means it is out of bounds.
    /// prefer not to return and return Coord instead, wonder if there's a way
    pub fn offset_by(&self, x: i32, y: i32) -> Coord {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
    pub fn is_within_grid(&self, grid_limit: &Coord) -> bool {
        self.x >= 0 && self.y >= 0 && grid_limit.x >= self.x && grid_limit.y >= self.y
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn resolve_token(token: char) -> Direction {
        match token {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            unexpected => panic!("Invalid direction token received {}", unexpected),
        }
    }
    pub fn next_dir_90deg_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn rotate_90deg_clockwise(&mut self) {
        *self = self.next_dir_90deg_clockwise()
    }
}
// const DIRECTION_TOKENS:
pub struct State {
    pub pos: Coord,
    pub visited: HashSet<Coord>,
    pub grid: Vec<Vec<char>>,
    pub dir: Direction,
    pub is_inside_grid: bool,
    pub grid_limit: Coord,
    pub obstacles: HashSet<Coord>,
}
impl State {
    pub fn new(
        grid: Vec<Vec<char>>,
        collected_data: Option<(Coord, Direction, HashSet<Coord>)>,
    ) -> State {
        let grid_limit: Coord = Coord {
            x: grid[0].len() as i32 - 1,
            y: grid.len() as i32 - 1,
        };
        let mut visited: HashSet<Coord> = HashSet::new();
        if collected_data.is_some() {
            let (pos, dir, obstacles) = collected_data.unwrap();
            visited.insert(pos.clone());
            State {
                pos,
                dir,
                grid,
                visited: HashSet::new(),
                is_inside_grid: true,
                grid_limit,
                obstacles,
            }
        } else {
            let mut obstacles: HashSet<Coord> = HashSet::new();
            let mut initial_coords_and_facing_dir: Option<(Coord, Direction)> = None;
            // find direction token and direction in the grid, and create the state once it's found
            for y in 0..=grid.len() - 1 {
                for x in 0..=grid[0].len() - 1 {
                    match grid[y][x] {
                        '.' => continue,
                        '#' => {
                            obstacles.insert(Coord::new(x, y));
                        }
                        direction_token => {
                            initial_coords_and_facing_dir =
                                Some((Coord::new(x, y), Direction::resolve_token(direction_token)))
                        }
                    }
                }
            }
            match initial_coords_and_facing_dir {
                None => panic!("No direction token found: {:?}", grid),
                Some((pos, dir)) => {
                    visited.insert(pos.clone());
                    State {
                        dir,
                        grid,
                        is_inside_grid: true,
                        grid_limit,
                        pos,
                        obstacles,
                        visited,
                    }
                }
            }
        }
    }
    pub fn get_curr_dir_offset(&self) -> (i32, i32) {
        match self.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn resolve_next_pos(&self) -> Coord {
        let (dx, dy) = self.get_curr_dir_offset();
        self.pos.offset_by(dx, dy)
    }
    pub fn move_to(&mut self, next_pos: Coord) {
        println!("> move from {} to {}", &self.pos, &next_pos);
        self.pos = next_pos;
        if self.pos.is_within_grid(&self.grid_limit) {
            self.visited.insert(self.pos.clone());
        } else {
            self.is_inside_grid = false;
        }
    }
    /// returns the next position that's okay
    pub fn rotate_as_needed_and_get_next_destination(&mut self) -> Coord {
        for _rotate_count in 0..4 {
            let next_pos: Coord = self.resolve_next_pos();
            if self.obstacles.contains(&next_pos) {
                println!(
                    "> rotate from {:?} to {:?}",
                    &self.dir,
                    &self.dir.next_dir_90deg_clockwise()
                );
                self.dir.rotate_90deg_clockwise();
            } else {
                return next_pos;
            }
        }
        panic!("Already rotated {} times", 4)
    }
}

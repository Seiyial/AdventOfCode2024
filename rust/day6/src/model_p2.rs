use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Visitations {
    // use vec for small set they said
    // so we will close this off so all the proper checking for uniq vec values are here
    data: HashMap<Coord, Vec<Direction>>,
}
impl Visitations {
    pub fn new() -> Visitations {
        Visitations {
            data: HashMap::new(),
        }
    }
    pub fn add_and_return_whether_looping(&mut self, coord: &Coord, dir: &Direction) -> bool {
        let mut already_exists = false;
        self.data
            .entry(coord.clone())
            .and_modify(|list| match list.contains(dir) {
                true => already_exists = true,
                false => list.push(dir.clone()),
            })
            .or_insert(vec![]);
        already_exists
    }
}

#[derive(Clone)]
pub struct State {
    pub grid: Vec<Vec<char>>,
    pub pos: Coord,
    pub init_pos: Coord,
    pub dir: Direction,
    pub init_dir: Direction,
    pub visitations: Visitations,
    pub init_visitations: Visitations,
    pub is_inside_grid: bool,
    pub grid_limit: Coord,
    pub obstacles: HashSet<Coord>,
    pub added_obstruction: Option<Coord>,
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
        let mut visitations: Visitations = Visitations::new();
        if collected_data.is_some() {
            let (pos, dir, obstacles) = collected_data.unwrap();
            State {
                init_pos: pos.clone(),
                pos,
                init_dir: dir.clone(),
                dir,
                grid,
                init_visitations: visitations.clone(),
                visitations,
                is_inside_grid: true,
                grid_limit,
                obstacles,
                added_obstruction: None,
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
                    visitations.add_and_return_whether_looping(&pos, &dir);
                    State {
                        grid,
                        init_dir: dir.clone(),
                        dir,
                        init_pos: pos.clone(),
                        pos,
                        is_inside_grid: true,
                        grid_limit,
                        obstacles,
                        init_visitations: visitations.clone(),
                        visitations,
                        added_obstruction: None,
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
    pub fn move_to_and_return_whether_looping(&mut self, next_pos: Coord) -> bool {
        // println!("> move from {} to {}", &self.pos, &next_pos);
        self.pos = next_pos;
        if self.pos.is_within_grid(&self.grid_limit) {
            self.visitations
                .add_and_return_whether_looping(&self.pos, &self.dir)
        } else {
            self.is_inside_grid = false;
            false
        }
    }
    /// returns the next position that's okay
    pub fn rotate_as_needed_and_get_next_destination(&mut self) -> Coord {
        for _rotate_count in 0..4 {
            let next_pos: Coord = self.resolve_next_pos();
            if self.obstacles.contains(&next_pos) {
                // println!(
                //     "> rotate from {:?} to {:?}",
                //     &self.dir,
                //     &self.dir.next_dir_90deg_clockwise()
                // );
                self.dir.rotate_90deg_clockwise();
            } else {
                return next_pos;
            }
        }
        panic!("Already rotated {} times", 4)
    }
    pub fn reset_with_new_obstruction_pos(&mut self, new_obstruction: Coord) {
        match &self.added_obstruction {
            None => (),
            Some(coord) => {
                self.obstacles.remove(&coord);
                self.added_obstruction = None
            }
        }
        self.pos = self.init_pos.clone();
        self.dir = self.init_dir.clone();
        self.obstacles.insert(new_obstruction.clone());
        self.added_obstruction = Some(new_obstruction);
        self.is_inside_grid = true;
        self.visitations = self.init_visitations.clone();
    }
}

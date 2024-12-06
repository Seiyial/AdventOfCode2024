pub mod model_p1;
pub mod model_p2;
pub mod model_p2_v2;

pub mod common {
    pub fn to_2d_char_array(input: String) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }
}

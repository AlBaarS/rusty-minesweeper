use rand::prelude::*;
use rand::{distributions::Uniform, Rng};
use itertools::*;

mod two_d_vector;
use two_d_vector::TwoDVector;

// Building up the board
fn generate_seed() -> u64 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen()
}

fn generate_mine_indices(number_of_squares: u32, seed: u64) -> Vec<u32> {
    let mut rng_seeded: StdRng = StdRng::seed_from_u64(seed);
    let range = Uniform::from(0..number_of_squares);
    let number_to_generate: u32 = number_of_squares * 100;
    let mine_density: u32 = number_of_squares / 5;
    let values: Vec<u32> = range.sample_iter(&mut rng_seeded).take(number_to_generate.try_into().unwrap()).unique().take(mine_density.try_into().unwrap()).sorted().collect();
    return values;
}



pub struct GameState {
    mines: TwoDVector<bool>,
    vicinity: TwoDVector<u8>,
    revealed: TwoDVector<bool>,
    flagged: TwoDVector<bool>,
}

impl GameState {
    fn new(mines: TwoDVector<bool>, vicinity: TwoDVector<u8>, revealed: TwoDVector<bool>, flagged: TwoDVector<bool>) -> Self {
        Self {
            mines,
            vicinity,
            revealed,
            flagged,
        }
    }

    pub fn generate_mines(board_size: usize, seed: u64) -> TwoDVector<bool> {
        let number_of_squares: u32 = (board_size * board_size).try_into().unwrap();
        let mine_indices: Vec<u32> = generate_mine_indices(number_of_squares, seed);
        let mine_vector: Vec<bool> = (0..number_of_squares)
            .map(|x: u32| mine_indices.contains(&x))
            .collect();
        let board: TwoDVector<bool> = TwoDVector::new(mine_vector, board_size);
        return board;
    }

    // pub fn calculate_vicinity(board: TwoDVector<bool>) -> TwoDVector<u8> {

    // }
}



fn main() {}

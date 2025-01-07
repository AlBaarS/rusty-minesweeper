use rand::prelude::*;
use rand::{distributions::Uniform, Rng};
use itertools::*;
use rusty_minesweeper::domain::logic::two_d_vector::TwoDVector;

// Building up the board
pub fn generate_seed() -> u64 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen()
}

pub fn generate_mine_indices(number_of_squares: u32, seed: u64) -> Vec<u32> {
    let mut rng_seeded: StdRng = StdRng::seed_from_u64(seed);
    let range = Uniform::from(0..number_of_squares);
    let number_to_generate: u32 = number_of_squares * 100;
    let mine_density: u32 = number_of_squares / 5;
    let values: Vec<u32> = range.sample_iter(&mut rng_seeded).take(number_to_generate.try_into().unwrap()).unique().take(mine_density.try_into().unwrap()).sorted().collect();
    return values;
}

// fn vector_to_matrix(vector: Vec<T>, matrix_size: u32) -> Vec<Vec<T>> {
    
// }



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

    fn generate_mines(board_size: u32, seed: u64) -> TwoDVector<bool> {
        let number_of_squares: u32 = board_size^2;
        let mine_indices: Vec<u32> = generate_mine_indices(number_of_squares, seed);
        let mine_vector: Vec<bool> = (0..number_of_squares)
            .map(|x: u32| mine_indices.contains(&x))
            .collect();
        let board: TwoDVector<bool> = TwoDVector::new(mine_vector, board_size);
        return board;
    }
}



fn main() {
    let seed: u64 = 1234567890;
    println!("{:?}", generate_mine_indices(255, seed));
    println!("{}", generate_mine_indices(255, seed).len());
}
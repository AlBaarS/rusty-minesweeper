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


#[cfg(test)]
mod tests {
    use crate::GameState;

    #[test]
    fn assert_board_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        let test_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, true], 
            vec![false, false, false, false, false], 
            vec![false, false, true, false, false], 
            vec![true, true, false, true, false], 
            vec![false, false, false, false, false]
        ];
        assert_eq!(GameState::generate_mines(5, test_seed).get_vec(), test_field);
    }
    // #[test]
    // fn test_if_mine_vicinity_function_works_correctly() -> () {
    //     let test_seed: u64 = 1234567890;
    //     let test_field: Vec<Vec<u8>> = vec![
    //         vec![0,0,0,1,9],
    //         vec![0,1,1,2,1],
    //         vec![2,3,9,2,1],
    //         vec![9,9,3,9,1],
    //         vec![2,2,2,1,1]
    //     ];
    //     let board: TwoDVector<u8> = GameState::generate_mines(5, test_seed);
    //     assert_eq!(GameState::calculate_vicinity(board).get_vec(), test_field);
    // }
}

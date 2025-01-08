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

    pub fn generate_mines(board_size: u8, seed: u64) -> TwoDVector<bool> {
        let number_of_squares: u32 = (board_size * board_size).try_into().unwrap();
        let mine_indices: Vec<u32> = generate_mine_indices(number_of_squares, seed);
        let mine_vector: Vec<bool> = (0..number_of_squares)
            .map(|x: u32| mine_indices.contains(&x))
            .collect();
        let board: TwoDVector<bool> = TwoDVector::new(mine_vector, board_size);
        return board;
    }

    fn calculate_vicinity(board: TwoDVector<bool>, x: i32, y: i32) -> u8 {
        // get elements neighbouring coordinate, count trues, return
        let mut nearby_squares: Vec<bool> = Vec::new();
        for xs in (x - 1..x + 2) {
            for ys in (y - 1..y + 2) {
                if (ys < board.get_size().into() && ys >= 0) && (xs < board.get_size().into() && xs >= 0) {
                    nearby_squares.push(board.get_element(xs.try_into().unwrap(), ys.try_into().unwrap()));
                }
            }
        };
        return nearby_squares.iter().filter(|&n| *n).count().try_into().unwrap();
    }

    pub fn make_vicinity_table(board: TwoDVector<bool>) -> TwoDVector<u8> {
        let board_vec: Vec<Vec<bool>> = board.clone().get_vec();
        let size: usize = board_vec.len();
        let mut vicinity: Vec<u8> = Vec::new();
        for (x, row) in board_vec.iter().enumerate() {
            for (y, elem) in row.iter().enumerate() {
                if *elem {
                    vicinity.push(9);
                } else {
                    vicinity.push(GameState::calculate_vicinity(board.clone(), x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        };
        TwoDVector::new(vicinity, size.try_into().unwrap())
    }
}



fn main() {}


#[cfg(test)]
mod tests {
    use crate::{two_d_vector::TwoDVector, GameState};

    #[test]
    fn assert_board_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        let reference_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, true], 
            vec![false, false, false, false, false], 
            vec![false, false, true, false, false], 
            vec![true, true, false, true, false], 
            vec![false, false, false, false, false]
        ];
        assert_eq!(GameState::generate_mines(5, test_seed).get_vec(), reference_field);
    }

    #[test]
    fn test_if_mine_vicinity_function_works_correctly() -> () {
        let test_seed: u64 = 1234567890;
        let reference_field: Vec<Vec<u8>> = vec![
            vec![0,0,0,1,9],
            vec![0,1,1,2,1],
            vec![2,3,9,2,1],
            vec![9,9,3,9,1],
            vec![2,2,2,1,1]
        ];
        let board: TwoDVector<bool> = GameState::generate_mines(5, test_seed);
        assert_eq!(GameState::make_vicinity_table(board).get_vec(), reference_field);
    }
}

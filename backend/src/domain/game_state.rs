use itertools::*;

mod two_d_vector;
use two_d_vector::TwoDVector;

mod game_generation;
use game_generation::GameGeneration;


pub struct GameState {
    mines: TwoDVector<bool>,
    vicinity: TwoDVector<u8>,
    revealed: TwoDVector<bool>,
    flagged: TwoDVector<bool>,
}

impl GameState {
    // Constructor
    fn new(mines: TwoDVector<bool>, vicinity: TwoDVector<u8>, revealed: TwoDVector<bool>, flagged: TwoDVector<bool>) -> Self {
        Self {
            mines,
            vicinity,
            revealed,
            flagged,
        }
    }

    // Getters
    pub fn get_mines(&self) -> &TwoDVector<bool> {
        return &self.mines;
    }

    pub fn get_vicinity(&self) -> &TwoDVector<u8> {
        return &self.vicinity;
    }

    pub fn get_revealed(&self) -> &TwoDVector<bool> {
        return &self.revealed;
    }

    pub fn get_flagged(&self) -> &TwoDVector<bool> {
        return &self.flagged;
    }

    // Functions
    fn generate_mines(size: u8, seed: u64) -> TwoDVector<bool> {
        let number_of_squares: u32 = (size * size).try_into().unwrap();
        let mine_indices: Vec<u32> = GameGeneration::generate_mine_indices(size.into(), seed);
        let mine_vector: Vec<bool> = (0..number_of_squares)
            .map(|x: u32| mine_indices.contains(&x))
            .collect();
        let board: TwoDVector<bool> = TwoDVector::new(mine_vector, size);
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

    fn make_vicinity_table(board: TwoDVector<bool>) -> TwoDVector<u8> {
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

    pub fn generate_starting_state(size: u8, seed: u64) -> GameState {
        let board: TwoDVector<bool> = GameState::generate_mines(size, seed);
        let vicinity: TwoDVector<u8> = Self::make_vicinity_table(board.clone());
        let revealed: TwoDVector<bool> = TwoDVector::new(repeat_n(false, (size * size).try_into().unwrap()).collect(), size);
        let flagged: TwoDVector<bool> = TwoDVector::new(repeat_n(false, (size * size).try_into().unwrap()).collect(), size);
        return GameState::new(board, vicinity, revealed, flagged);
    }
}



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

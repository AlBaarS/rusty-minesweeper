use std::cmp::Reverse;

use itertools::*;
use serde::Serialize;

use super::base_components::{game_generation::GameGeneration, two_d_vector::TwoDVector};

#[derive(Serialize, Debug, Clone)]
pub struct Board {
    mines: TwoDVector<bool>,
    vicinity: TwoDVector<u8>,
    revealed: TwoDVector<bool>,
    flagged: TwoDVector<bool>,
}

impl Board {
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


    // Functions for board initialization
    fn generate_mines(size: u8, seed: u64) -> TwoDVector<bool> {
        let size_u32: u32 = size.into();
        let number_of_squares: u32 = size_u32 * size_u32;
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
        for ys in (y - 1..y + 2) {
            for xs in (x - 1..x + 2) {
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
        for (y, row) in board_vec.iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if *elem {
                    vicinity.push(9);
                } else {
                    vicinity.push(Board::calculate_vicinity(board.clone(), x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        };
        TwoDVector::new(vicinity, size.try_into().unwrap())
    }

    pub fn generate_starting_state(size: u8, seed: u64) -> Board {
        let size_usize: usize = size.into();
        let mines: TwoDVector<bool> = Board::generate_mines(size, seed);
        let vicinity: TwoDVector<u8> = Self::make_vicinity_table(mines.clone());
        let revealed: TwoDVector<bool> = TwoDVector::new(repeat_n(false, size_usize * size_usize).collect(), size);
        let flagged: TwoDVector<bool> = TwoDVector::new(repeat_n(false, size_usize * size_usize).collect(), size);
        return Board::new(mines, vicinity, revealed, flagged);
    }


    // Functions for performing an action (clicking a square or placing/removing a flag)
    pub fn reveal_square(&self, x: usize, y: usize) -> Board {
        let mines: TwoDVector<bool> = self.get_mines().clone();
        let vicinity: TwoDVector<u8> = self.get_vicinity().clone();
        let revealed: TwoDVector<bool> = self.get_revealed().change_element(x, y, true);
        let flagged: TwoDVector<bool> = self.get_flagged().clone();
        return Board::new(mines, vicinity, revealed, flagged);
    }

    pub fn flag_unflag_square(&self, x: usize, y: usize) -> Board {
        let mines: TwoDVector<bool> = self.get_mines().clone();
        let vicinity: TwoDVector<u8> = self.get_vicinity().clone();
        let revealed: TwoDVector<bool> = self.get_revealed().clone();
        let flagged: TwoDVector<bool> = self.get_flagged().change_element(x, y, !self.get_flagged().get_element(x, y));
        return Board::new(mines, vicinity, revealed, flagged);
    }
}


#[allow(dead_code)]
fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_mine_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        let test_size: u8 = 5;
        let reference_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, true], 
            vec![false, false, false, false, false], 
            vec![false, false, true, false, false], 
            vec![true, true, false, true, false], 
            vec![false, false, false, false, false]
        ];
        assert_eq!(Board::generate_mines(test_size, test_seed).get_vec(), reference_field);
    }

    #[test]
    fn test_if_mine_vicinity_function_works_correctly() -> () {
        let test_seed: u64 = 1234567890;
        let test_size: u8 = 5;
        let reference_field: Vec<Vec<u8>> = vec![
            vec![0,0,0,1,9],
            vec![0,1,1,2,1],
            vec![2,3,9,2,1],
            vec![9,9,3,9,1],
            vec![2,2,2,1,1]
        ];
        let board: TwoDVector<bool> = Board::generate_mines(test_size, test_seed);
        assert_eq!(Board::make_vicinity_table(board).get_vec(), reference_field);
    }

    #[test]
    fn test_if_square_is_revealed() -> () {
        let test_seed: u64 = 1234567890;
        let test_size: u8 = 5;
        let reference_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false]
        ];
        let board: Board = Board::generate_starting_state(test_size, test_seed);
        assert_eq!(board.reveal_square(3, 2).get_revealed().get_vec(), reference_field);
    }

    #[test]
    fn test_if_flag_is_placed() -> () {
        let test_seed: u64 = 1234567890;
        let test_size: u8 = 5;
        let reference_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false]
        ];
        let board: Board = Board::generate_starting_state(test_size, test_seed);
        assert_eq!(board.flag_unflag_square(0, 2).get_flagged().get_vec(), reference_field);
    }

    #[test]
    fn test_if_flag_is_removed() -> () {
        let test_seed: u64 = 1234567890;
        let test_size: u8 = 5;
        let reference_field: Vec<Vec<bool>> = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false]
        ];
        let board: Board = Board::generate_starting_state(test_size, test_seed);
        assert_eq!(board.flag_unflag_square(0, 2).flag_unflag_square(0, 2).get_flagged().get_vec(), reference_field);
    }
}

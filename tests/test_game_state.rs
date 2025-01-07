#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::game_state::GameState;
    use rusty_minesweeper::domain::logic::two_d_vector::TwoDVector;

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

fn main() {}

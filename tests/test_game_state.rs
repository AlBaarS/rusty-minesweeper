#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::game_state::GameState;
    use rusty_minesweeper::domain::logic::two_d_vector::TwoDVector;

    // [2, 6, 7, 9, 15, 18, 21, 25, 34, 41, 43, 50, 52, 67, 70, 78, 84, 85, 89, 90, 97, 102, 104, 111, 114, 119, 122, 124, 125, 
    // 129, 131, 138, 143, 160, 165, 166, 168, 175, 183, 190, 191, 192, 217, 231, 234, 237, 240, 243, 248, 249, 254]

    #[test]
    fn assert_board_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        let test_field: Vec<Vec<bool>> = vec![
            vec![false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true], 
            vec![false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false], 
            vec![false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false], 
            vec![false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false], 
            vec![false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true], 
            vec![false, false, false, false, true, true, false, false, false, true, true, true, false, false, false, false], 
            vec![false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true], 
            vec![false, false, false, true, false, false, false, true, false, false, true, false, true, false, true, false], 
            vec![false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false], 
            vec![true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false], 
            vec![true, true, false, false, false, false, true, true, false, true, false, false, false, false, false, false], 
            vec![true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true], 
            vec![true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false], 
            vec![false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false], 
            vec![false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false], 
            vec![false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true]
        ];
        assert_eq!(GameState::generate_mines(16, test_seed).get_vec(), test_field);
    }
}

fn main() {}

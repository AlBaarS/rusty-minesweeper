#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::game_state::{self};

    //[2, 6, 9, 15, 25, 34, 41, 43, 50, 52, 67, 78, 84, 85, 89, 90, 97, 102, 104, 111, 119, 122, 124, 125, 129, 
    // 138, 143, 160, 165, 166, 175, 183, 191, 217, 231, 234, 240, 248, 249, 254]

    #[test]
    fn assert_board_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        let test_field: Vec<Vec<bool>> = [
            [false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false],
            [false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false],
            [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false],
            [false, false, false, false, true, true, false, false, false, true, true, false, false, false, false, false],
            [false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, true, false, false, true, false, true, true, false, false],
            [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true],
            [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            [true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false],
            [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false],
            [true, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false]
        ]
    }
}
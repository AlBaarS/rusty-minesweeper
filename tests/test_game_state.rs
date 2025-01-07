#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::game_state::{self, generate_mine_indices};

    // [2, 6, 7, 9, 15, 18, 21, 25, 34, 41, 43, 50, 52, 67, 70, 78, 84, 85, 89, 90, 97, 102, 104, 111, 114, 119, 122, 124, 125, 
    // 129, 131, 138, 143, 160, 165, 166, 168, 175, 183, 190, 191, 192, 217, 231, 234, 237, 240, 243, 248, 249, 254]

    #[test]
    fn assert_board_generation_is_consistent_with_seed() -> () {
        let test_seed: u64 = 1234567890;
        // let test_field: Vec<Vec<bool>> = vec![
        //     vec![false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        //     vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]
        // ];
        println!("{:?}", generate_mine_indices(255, test_seed));
        println!("{}", generate_mine_indices(255, test_seed).len());
    }
}

fn main() {}

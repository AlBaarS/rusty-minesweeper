use super::logic::game_state::Board;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Progress {
    Win,
    Lose,
    InProgress,
}

#[derive(Serialize, Debug)]
pub struct Play {
    game: Board,
    progress: Progress,
}

impl Play {
    pub fn new(seed: u64) -> Self {
        let game: Board = Board::generate_starting_state(16, seed);
        let progress: Progress = Progress::InProgress;
        return Self {
            game,
            progress,
        };
    }

    pub fn get_game(&self) -> &Board {
        return &self.game;
    }

    pub fn get_progress(&self) -> &Progress {
        return &self.progress;
    }

    pub fn test_import() -> String {
        return String::from("It works!");
    }
}


fn main() {
    let game: Play = Play::new(1234567890);
    println!("Game: ${:?}", game);
}

#[cfg(test)]
mod tests {
    use crate::domain::logic::game_state::Board;
    use super::Play;

    #[test]
    fn test_import_of_Play() -> () {
        assert_eq!(Play::test_import(), String::from("It works!"));
    }

    #[test]
    fn test_game_generation() -> () {
        let play: Play = Play::new(1234567890);
        let test_board: &Board = play.get_game();
        let ref_board: Board = Board::generate_starting_state(16, 1234567890);
        println!("{:?}", test_board);
        assert_eq!(test_board.get_mines().get_vec(), ref_board.get_mines().get_vec());
    }
}

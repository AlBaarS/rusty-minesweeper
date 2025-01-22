use super::logic::game_state::Board;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    Win,
    Lost,
    InProgress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub(crate) game: Board,
    pub(crate) progress: Progress,
}

impl Play {
    // Constructor
    pub fn new(seed: u64) -> Self {
        let game: Board = Board::generate_starting_state(16, seed);
        let progress: Progress = Progress::InProgress;
        return Self {
            game,
            progress,
        };
    }

    // Getters
    pub fn get_game(&self) -> &Board {
        return &self.game;
    }

    pub fn get_progress(&self) -> &Progress {
        return &self.progress;
    }

    pub fn test_import() -> String {
        return String::from("It works!");
    }

    // Game logic
    fn is_lost(&self, x: usize, y: usize) -> bool {
        return self.get_game().get_mines().get_element(x, y);
    }

    fn is_won(&self) -> bool {
        return self.get_game().get_revealed().get_vec() == self.get_game().invert_mines().get_vec();
    }

    pub fn autowin(&self) -> Play {
        return Play {
            game: Board {
                mines: self.get_game().get_mines(),
                vicinity: self.get_game().get_vicinity(),
                revealed: self.get_game().invert_mines(),
                flagged: self.get_game().get_flagged(),
            },
            progress: Progress::InProgress,
        };
    }

    pub fn play_square(&self, x: usize, y: usize) -> Play {
        let new_board: Board = self.get_game().reveal_square(x, y);
        if self.is_lost(x, y) {
            return Play {
                game: new_board.reveal_all(),       //to implement
                progress: Progress::Lost,
            };
        } else {       
            if self.is_won() {                      //to implement
                return Play {
                    game: new_board.reveal_all(),       //to implement
                    progress: Progress::Win,
                };
            } else {
                return Play {
                    game: new_board.reveal_all(),       //to implement
                    progress: Progress::InProgress,
                };
            };
        };
    }
}


fn main() {
    let game: Play = Play::new(1234567890);
    println!("Game: {:?}", game);
}

#[cfg(test)]
mod tests {
    use crate::domain::{logic::game_state::Board, play::Progress};
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

    #[test]
    fn test_if_square_with_mine_will_return_Lost_when_clicked() {
        let play: Play = Play::new(1234567890).play_square(2, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::Lost);
    }

    #[test]
    fn test_if_square_without_mine_will_not_return_Lost_when_clicked() {
        let play: Play = Play::new(1234567890).play_square(1, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::InProgress);
    }

    #[test]
    fn test_if_win_is_detected_when_all_non_mined_squares_are_revealed() {
        let play: Play = Play::new(1234567890).autowin().play_square(1, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::Win);
    }
}

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
    pub(crate) difficulty: u32,
}

impl Play {
    // Constructor
    pub fn new(seed: u64, difficulty: u32) -> Self {
        let game: Board = Board::generate_starting_state(16, seed, difficulty);
        let progress: Progress = Progress::InProgress;
        return Self {
            game,
            progress,
            difficulty
        };
    }

    // Getters
    pub fn get_game(&self) -> &Board {
        return &self.game;
    }

    pub fn get_progress(&self) -> &Progress {
        return &self.progress;
    }

    pub fn get_difficulty(&self) -> &u32 {
        return &self.difficulty;
    }

    pub fn test_import() -> String {
        return String::from("It works!");
    }

    // Game logic
    pub fn _autowin_reveal(&self) -> Play {     // For testing purposes only
        return Play {
            game: Board {
                mines: self.get_game().get_mines(),
                vicinity: self.get_game().get_vicinity(),
                revealed: self.get_game().invert_mines(),
                flagged: self.get_game().get_flagged(),
            },
            progress: Progress::InProgress,
            difficulty: self.difficulty,
        };
    }

    pub fn _autowin_flags(&self) -> Play {     // For testing purposes only
        return Play {
            game: Board {
                mines: self.get_game().get_mines(),
                vicinity: self.get_game().get_vicinity(),
                revealed: self.get_game().get_revealed(),
                flagged: self.get_game().get_mines(),
            },
            progress: Progress::InProgress,
            difficulty: self.difficulty,
        };
    }

    pub fn play_square(&self, x: usize, y: usize) -> Play {
        let new_board: Board = self.get_game().reveal_square(x, y);
        if new_board.get_mines().get_element(x, y) {
            return Play {
                game: new_board.reveal_all(),
                progress: Progress::Lost,
                difficulty: self.difficulty,
            };
        } else {       
            if new_board.all_non_mines_are_revealed() || new_board.all_mines_are_flagged() {
                return Play {
                    game: new_board.reveal_safe(),
                    progress: Progress::Win,
                    difficulty: self.difficulty,
                };
            } else {
                return Play {
                    game: new_board,
                    progress: Progress::InProgress,
                    difficulty: self.difficulty,
                };
            };
        };
    }

    pub fn flag_square(&self, x: usize, y: usize) -> Play {
        let new_board: Board = self.get_game().flag_unflag_square(x, y);
        if new_board.all_non_mines_are_revealed() || new_board.all_mines_are_flagged() {
            return Play {
                game: new_board.reveal_safe(),
                progress: Progress::Win,
                difficulty: self.difficulty,
            };
        } else {
            return Play {
                game: new_board,
                progress: Progress::InProgress,
                difficulty: self.difficulty,
            };
        };
    }
}



#[cfg(test)]
mod tests {
    use std::iter;
    use itertools::*;
    use crate::domain::{logic::{base_components::two_d_vector::TwoDVector, game_state::Board}, play::Progress};
    use super::Play;

    #[test]
    fn test_import_of_Play() -> () {
        assert_eq!(Play::test_import(), String::from("It works!"));
    }

    #[test]
    fn test_game_generation() -> () {
        let play: Play = Play::new(1234567890, 5);
        let test_board: &Board = play.get_game();
        let ref_board: Board = Board::generate_starting_state(16, 1234567890, 5);
        println!("{:?}", test_board);
        assert_eq!(test_board.get_mines().get_vec(), ref_board.get_mines().get_vec());
    }

    #[test]
    fn test_if_square_with_mine_will_return_Lost_when_clicked() {
        let play: Play = Play::new(1234567890, 5).play_square(2, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::Lost);
    }

    #[test]
    fn test_if_square_without_mine_will_not_return_Lost_when_clicked() {
        let play: Play = Play::new(1234567890, 5).play_square(1, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::InProgress);
    }

    #[test]
    fn test_if_all_squares_are_revealed_when_the_game_is_lost() {
        let play: Play = Play::new(1234567890, 5).play_square(2, 0);
        let revealed_board: TwoDVector<bool> = TwoDVector::new(iter::repeat(true).take(256).collect_vec(), 16);
        assert_eq!(play.get_game().get_revealed().get_vec(), revealed_board.get_vec());
    }

    #[test]
    fn test_if_win_is_detected_when_all_non_mined_squares_are_revealed() {
        let play: Play = Play::new(1234567890, 5)._autowin_reveal().play_square(1, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::Win);
    }

    #[test]
    fn test_if_flag_is_placed() {
        let play: Play = Play::new(1234567890, 5).flag_square(1, 3);
        assert!(play.get_game().get_flagged().get_element(1, 3));
    }

    #[test]
    fn test_if_win_is_detected_when_all_mines_are_flagged() {
        let play: Play = Play::new(1234567890, 5)._autowin_flags().play_square(1, 0);
        assert_eq!(play.get_progress().to_owned(), Progress::Win);
    }
}

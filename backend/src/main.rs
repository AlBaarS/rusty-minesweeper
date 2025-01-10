pub mod domain;
use domain::{base::game_generation::GameGeneration, game_state::GameState};

pub enum Progress {
    Win,
    Lose,
    InProgress,
}
pub struct Play {
    game: GameState,
    progress: Progress,
}

impl Play {
    pub fn new() -> Self {
        let game: GameState = GameState::generate_starting_state(16, GameGeneration::generate_seed());
        let progress: Progress = Progress::InProgress;
        return Self {
            game,
            progress,
        };
    }

    pub fn get_game(&self) -> &GameState {
        return &self.game;
    }

    pub fn get_progress(&self) -> &Progress {
        return &self.progress;
    }

    pub fn test_import() -> bool {
        return true;
    }
}

fn main() -> () {}

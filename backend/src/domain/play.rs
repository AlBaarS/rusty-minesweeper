use super::logic::{base_components::game_generation::GameGeneration, game_state::GameState};
use serde::Serialize;

#[derive(Serialize)]
pub enum Progress {
    Win,
    Lose,
    InProgress,
}

#[derive(Serialize)]
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

    pub fn test_import() -> String {
        return String::from("It works!");
    }
}


fn main() {}

#[cfg(test)]
mod tests {
    use super::Play;

    #[test]
    fn test_import_of_Play() -> () {
        assert_eq!(Play::test_import(), String::from("It works!"));
    }
}

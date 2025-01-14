use super::logic::game_state::GameState;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Progress {
    Win,
    Lose,
    InProgress,
}

#[derive(Serialize, Debug)]
pub struct Play {
    game: GameState,
    progress: Progress,
}

impl Play {
    pub fn new(seed: u64) -> Self {
        let game: GameState = GameState::generate_starting_state(16, seed);
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


fn main() {
    let game: Play = Play::new(1234567890);
    println!("Game: ${:?}", game);
}

#[cfg(test)]
mod tests {
    use super::Play;

    #[test]
    fn test_import_of_Play() -> () {
        assert_eq!(Play::test_import(), String::from("It works!"));
    }

    #[test]
    fn test_game_generation() -> () {
        let game: Play = Play::new(1234567890);
        println!("Game: ${:?}", game);
    }
}

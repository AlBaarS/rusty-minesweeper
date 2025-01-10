use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

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


#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users));

    println!("Running on http://localhost:3001");
    // Start Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::super::backend::main::Play;

    #[test]
    fn test_import_of_Play() -> () {
        assert!(Play::test_import());
    }
}

use axum::{extract::{self, Path}, Json};
use serde_json::json;

use crate::domain::play::Play;

pub async fn test_import() -> Json<serde_json::Value> {
    println!{"API call received for test_import()"};
    return Json(json!({ "message": Play::test_import() }));
}

pub async fn start_game(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for start_game() with body ${}", payload);

    let seed: u64 = match payload["seed"].as_u64() {
        Some(s) => s,
        None => return Json(json!({"playboard" : "invalid input"})),
    };

    println!("Seed registered: ${}. Returning a new board to front-end.", seed);
    return Json(json!({ "playboard": Play::new(seed)}));
}
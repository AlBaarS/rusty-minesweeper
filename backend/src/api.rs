use axum::{extract::Path, Json};
use serde_json::json;

use crate::domain::play::Play;

pub async fn test_import() -> Json<serde_json::Value> {
    println!{"API call received for: test_import()"};
    return Json(json!({ "message": Play::test_import() }));
}

pub async fn start_game(Path(seed): Path<u64>) -> Json<serde_json::Value> {
    println!{"API call received for: start_game()"};
    return Json(json!({ "playboard": Play::new(seed)}))
}
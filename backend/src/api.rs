use axum::{Router, routing::get, Json};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};

use crate::domain::play::Play;

pub async fn test_import() -> Json<serde_json::Value> {
    println!{"API call received for: test_import()"}
    return Json(json!({ "message": Play::test_import() }));
}
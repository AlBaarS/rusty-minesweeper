mod api;
mod app;
mod domain;

use axum::{
    body::Body,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Define the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods(vec![Method::GET, Method::POST]); // Allow GET and POST methods

    // Define API routes
    let api_router = Router::new()
        .route("/minesweeper", get(api::test_import));

    // Create the application and add routes
    let app = Router::new()
        .nest("/api", api_router)
        .layer(cors); // Attach the CORS layer

    // Start the server
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
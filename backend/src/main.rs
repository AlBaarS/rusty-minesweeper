mod api;
mod domain;
mod db_connection;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Define the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_headers(Any) // Allow all types of headers
        .allow_methods(vec![Method::GET, Method::POST]); // Allow GET and POST methods

    // Define API routes
    let api_router = Router::new()
        .route("/test", get(api::test_import))
        .route("/create", post(api::start_game))
        .route("/play", post(api::play))
        .route("/flag", post(api::flag))
        .route("/find", post(api::find))
        .route("/continue", post(api::continue_game));

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
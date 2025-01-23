use axum::{extract::{self, Path}, Json};
use serde_json::json;
use mongodb::{
    bson::doc, 
    options::{
        ClientOptions, 
        ServerApi, 
        ServerApiVersion
    }, 
    Client
};
use crate::domain::{
    logic::game_state::Board, 
    play::{
        Play, 
        Progress
    }
};
use crate::db_connection::create_and_connect::*;

// Establishing database


// Testing calls
pub async fn test_import() -> Json<serde_json::Value> {
    println!("API call received for test_import()");
    return Json(json!({ "message": Play::test_import() }));
}

// Start game
pub async fn start_game(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for start_game() with body {}", payload);

    let seed: u64 = match payload["seed"].as_u64() {
        Some(s) => s,
        None => return Json(json!({"playboard" : "invalid input"})),
    };

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"playboard" : "invalid input"})),
    }.try_into().unwrap();

    println!("Seed registered: {}. Returning a new board to front-end.", seed);
    let game: Play = Play::new(seed);
    store_gamestate(game.clone(), user).await;
    return Json(json!({ "playboard": game}));
}

// Perform a move
pub async fn play(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for play() with body {}", payload);

    let user: String = payload["user"].to_string();
    let x: usize = match usize::try_from(payload["x"].as_u64().unwrap()) {
        Ok(x) => x,
        Err(e) => panic!("Unable to convert x to usize: {}", e)
    };
    let y: usize = match usize::try_from(payload["y"].as_u64().unwrap()) {
        Ok(y) => y,
        Err(e) => panic!("Unable to convert y to usize: {}", e)
    };
    
    let game_old: Play = fetch_gamestate(user.clone()).await;
    let game_new: Play = game_old.play_square(x, y);

    if game_new.get_progress().to_owned() == Progress::InProgress {
        update_gamestate(user, game_new.clone()).await;
    } else {
        delete_gamestate(user).await;
    }

    return Json(json!({ "playboard": game_new}));
}

// Flag a square
pub async fn flag(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for flag() with body {}", payload);

    let user: String = payload["user"].to_string();
    let x: usize = match usize::try_from(payload["x"].as_u64().unwrap()) {
        Ok(x) => x,
        Err(e) => panic!("Unable to convert x to usize: {}", e)
    };
    let y: usize = match usize::try_from(payload["y"].as_u64().unwrap()) {
        Ok(y) => y,
        Err(e) => panic!("Unable to convert y to usize: {}", e)
    };
    
    let game_old: Play = fetch_gamestate(user.clone()).await;
    let game_new: Play = game_old.flag_square(x, y);

    if game_new.get_progress().to_owned() == Progress::InProgress {
        update_gamestate(user, game_new.clone()).await;
    } else {
        delete_gamestate(user).await;
    }

    return Json(json!({ "playboard": game_new}));
}

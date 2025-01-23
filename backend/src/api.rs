use axum::Json;
use serde_json::json;
use crate::domain::play::{
    Play, 
    Progress
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
        None => return Json(json!({"playboard" : "invalid seed"})),
    };

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"playboard" : "invalid user"})),
    }.try_into().unwrap();

    println!("User registered: {}. Searching for existing game in database.", user);
    let saved_game: bool = find_game(user.clone()).await;
    if saved_game {
        delete_gamestate(user.clone()).await;
    }

    println!("Seed registered: {}. Returning a new board to front-end.", seed);
    let game: Play = Play::new(seed);
    store_gamestate(game.clone(), user).await;
    return Json(json!({ "playboard": game}));
}

// Perform a move
pub async fn play(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for play() with body {}", payload);

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"playboard" : "invalid user"})),
    }.try_into().unwrap();
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
        update_gamestate(user.clone(), game_new.clone()).await;
        println!("Gamestate for {} updated after move", user);
    } else {
        delete_gamestate(user.clone()).await;
        println!("Game over for {}, end state: {:?}", user, game_new.get_progress());
    }

    println!("New gamestate generated, returning to user");
    return Json(json!({ "playboard": game_new}));
}

// Flag a square
pub async fn flag(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for flag() with body {}", payload);

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"playboard" : "invalid user"})),
    }.try_into().unwrap();
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

    update_gamestate(user.clone(), game_new.clone()).await;
    println!("Gamestate for {} updated after flag", user);

    return Json(json!({ "playboard": game_new}));
}

pub async fn find(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for find() with body {}", payload);

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"game" : "invalid user"})),
    }.try_into().unwrap();

    let game_presence: bool = find_game(user.clone()).await;
    println!("Found game for user {}: {}", user, game_presence);
    return Json(json!({"game" : game_presence}))
}

pub async fn continue_game(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("API call received for continue_game() with body {}", payload);

    let user: String = match payload["user"].as_str() {
        Some(user) => user,
        None => return Json(json!({"playboard" : "invalid user"})),
    }.try_into().unwrap();

    let game: Play = fetch_gamestate(user).await;
    println!("Game retrieved from database, returning to user");
    return Json(json!({ "playboard": game}));
}

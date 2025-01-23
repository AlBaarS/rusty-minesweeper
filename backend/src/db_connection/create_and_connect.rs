use mongodb::{
    bson::{self, Bson, Document, doc}, 
    options::{
        ClientOptions, 
        ServerApi, 
        ServerApiVersion
    }, 
    Client
};

use crate::domain::play::Play;

const DB_URI: &str = "mongodb://127.0.0.1:27017/";

async fn get_connection() -> mongodb::Collection<Document>  {
    // Establish connection to MongoDB
    let mut client_options: ClientOptions =
        match ClientOptions::parse(DB_URI).await {
            Ok(client_options) => client_options,
            Err(e) => panic!("Unable to obtain client options: {}", e)
        };

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api: ServerApi = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client: Client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => panic!("Unable to create client with options: {}", e)
    };

    // Get the 'gamestates' collection from the 'rusty_minesweeper' database
    let gamestates: mongodb::Collection<Document> = client.database("rusty_minesweeper").collection("gamestates");

    println!("Connection to database established");
    return gamestates;
}

pub async fn store_gamestate(game: Play, user: String) -> () {

    println!("Call to store gamestate...");

    // Set up the document to send along
    let game_bson: Bson = match bson::to_bson(&game) {
        Ok(game_bson) => game_bson,
        Err(e) => panic!("Conversion of gamestate to BSON failed: {}", e)
    };
    let outgoing_doc: bson::Document = doc! {
        "user": user,
        "gamestate": game_bson,
    };

    // Get the 'gamestates' collection from the 'rusty_minesweeper' database
    let gamestates: mongodb::Collection<Document> = get_connection().await;

    // Insert our gamestate into the database
    let insert_result: mongodb::results::InsertOneResult = match gamestates.insert_one(outgoing_doc.clone(), None).await {
        Ok(insert_result) => insert_result,
        Err(e) => panic!("Failed to insert data into MongoDB: {}", e)
    };
    println!("Database insertion successful. New document ID: {}", insert_result.inserted_id);
}



pub async fn fetch_gamestate(user: String) -> Play {

    println!("Call to fetch gamestate...");

    // Establish connection to the database
    let gamestates: mongodb::Collection<Document> = get_connection().await;

    // Set up the document to filter
    let user_prefix_rm = match user.strip_prefix("\"") {
        Some(user_prefix_rm) => user_prefix_rm,
        None => &user
    };
    let user_formatted = match user_prefix_rm.strip_suffix("\"") {
        Some(user_formatted) => user_formatted,
        None => &user_prefix_rm
    };
    let filter_doc: Document = doc! {
        "user": user_formatted
    };

    // Find the correct document for a specific user
    let game_opt: Option<Document> = match gamestates.find_one(
        filter_doc,  
        None,  
    ).await {
        Ok(game_opt) => game_opt,
        Err(e) => panic!("Error: Cannot find document for {}: {}", &user, e)
    };
    let game_doc: Document = match game_opt {
        Some(game_doc) => game_doc,
        _ => panic!("Error: Empty Doc found for {}", &user)
    };

    // Convert the document into a Play object
    let game_element: &Bson = match game_doc.get("gamestate") {
        Some(game_element) => game_element,
        _ => panic!("Error: Cannot find gamestate in document")
    };
    let game: Play = match bson::from_bson(game_element.clone()) {
        Ok(game) => game,
        Err(e) => panic!("Error: Cannot convert document to Play object: {}", e)
    };

    println!("Game of {} fetched successfully!", &user);

    return game;
}

pub async fn find_game(user: String) -> bool {

    println!("Call to find game for user {}", &user);

    // Establish connection to the database
    let gamestates: mongodb::Collection<Document> = get_connection().await;

    // Set up the document to filter
    let user_prefix_rm = match user.strip_prefix("\"") {
        Some(user_prefix_rm) => user_prefix_rm,
        None => &user
    };
    let user_formatted = match user_prefix_rm.strip_suffix("\"") {
        Some(user_formatted) => user_formatted,
        None => &user_prefix_rm
    };
    let filter_doc: Document = doc! {
        "user": user_formatted
    };

    // Find the correct document for a specific user
    let game_opt: Option<Document> = match gamestates.find_one(
        filter_doc,  
        None,  
    ).await {
        Ok(game_opt) => game_opt,
        Err(_e) => return false
    };
    let _game_doc: Document = match game_opt {
        Some(_game_doc) => return true,
        _ => return false
    };
}

pub async fn update_gamestate(user: String, game: Play) -> () {

    println!("Call to update gamestate...");

    // Establish connection to the database
    let gamestates: mongodb::Collection<Document> = get_connection().await;

    // Set up the documents to filter and send along
    let user_prefix_rm = match user.strip_prefix("\"") {
        Some(user_prefix_rm) => user_prefix_rm,
        None => &user
    };
    let user_formatted = match user_prefix_rm.strip_suffix("\"") {
        Some(user_formatted) => user_formatted,
        None => &user_prefix_rm
    };
    let filter_doc: Document = doc! {
        "user": user_formatted
    };
    let game_bson: Bson = match bson::to_bson(&game) {
        Ok(game_bson) => game_bson,
        Err(e) => panic!("Conversion of gamestate to BSON failed: {}", e)
    };
    let update_doc: Document = doc! {
        "$set" : { "gamestate": game_bson }
    };

    // Update the correct document for a specific user
    let update_result: mongodb::results::UpdateResult = match gamestates.update_one(
        filter_doc,
        update_doc,
        None,  
    ).await {
        Ok(update_result) => update_result,
        Err(e) => panic!("Error: Cannot find document for {}: {}", &user, e)
    };

    println!("Database update successfully. Updated elements: {}", update_result.modified_count);
}

pub async fn delete_gamestate(user: String) -> () {
    println!("Call to delete gamestate...");

    // Establish connection to the database
    let gamestates: mongodb::Collection<Document> = get_connection().await;

    // Set up the document to filter
    let user_prefix_rm = match user.strip_prefix("\"") {
        Some(user_prefix_rm) => user_prefix_rm,
        None => &user
    };
    let user_formatted = match user_prefix_rm.strip_suffix("\"") {
        Some(user_formatted) => user_formatted,
        None => &user_prefix_rm
    };
    let filter_doc: Document = doc! {
        "user": user_formatted
    };

    // Delete the correct document for a specific user
    let game_delete_results: mongodb::results::DeleteResult = match gamestates.delete_many(
        filter_doc,  
        None,  
    ).await {
        Ok(game_opt) => game_opt,
        Err(e) => panic!("Error: Cannot find document for {}: {}", &user, e)
    };

    println!("Successfully deleted {} games of {}!", game_delete_results.deleted_count, &user);
}

mod server;
mod db;

use crate::server::websocket::chat_server;
use futures::{SinkExt, StreamExt};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use warp::Filter;
use db::*;
use crate::server::http::http_server;

// #[tokio::main]
#[rocket::main]
async fn main() {
    // Prepare DB
    let conn = Connection::open("my_database.db").unwrap();
    mock_data::prepare_db(&conn);

    // Start websocket server
    tokio::spawn(async move {
        chat_server::init().await;
    });

    // Launch the Rocket HTTP server concurrently
    http_server::init().await;
}


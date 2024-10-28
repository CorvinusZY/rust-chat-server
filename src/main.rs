mod server;
mod db;

use crate::server::chat_server;
use futures::{SinkExt, StreamExt};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use warp::Filter;
use db::*;

#[tokio::main]
async fn main() {
    // Prepare DB
    let conn = Connection::open("my_database.db").unwrap();
    mock_data::prepare_db(&conn);

    // Start websocket server
    chat_server::start_server().await;
}


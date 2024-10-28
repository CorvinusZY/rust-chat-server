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
    user::create_table(&conn);
    user::inspect_users(&conn);

    // Start websocket server
    chat_server::start_server().await;
}

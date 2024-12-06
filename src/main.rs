mod data;
mod db;
mod server;

use crate::server::http::http_server;
use crate::server::websocket::chat_server;
use db::*;
use rusqlite::Connection;

// #[tokio::main]
#[rocket::main]
async fn main() {
    // Prepare DB
    let conn = Connection::open("my_database.db").unwrap();
    conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
    mock_data::prepare_db(&conn);

    // Start websocket server
    tokio::spawn(async move {
        chat_server::init().await;
    });

    // Launch the Rocket HTTP server concurrently
    http_server::init().await;
}

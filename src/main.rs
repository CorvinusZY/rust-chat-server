mod server;

use crate::server::chat_server;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[tokio::main]
async fn main() {
    chat_server::start_server().await;
}

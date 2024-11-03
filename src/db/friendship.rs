use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use crate::db::message::MessageType;

// tracks direct friendship in both directions
#[derive(Debug, Serialize, Deserialize)]
pub struct Friendship {
    pub id: i64,
    pub from_username: String,
    pub to_username: String,
}


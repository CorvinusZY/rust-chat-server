use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rusqlite::Connection;
use crate::db::message;
use crate::db::message::Message;

// Get chat history for direct messaging
#[derive(Deserialize)]
struct GetDirectChatHistoryInput {
    username_a: String,
    username_b: String,
}

#[derive(Serialize)]
struct GetDirectChatHistoryOutput {
    messages: Vec<Message>,
}

#[get("/chat-history", data = "<data>")]
pub async fn get_chat_history(data: Json<GetDirectChatHistoryInput>) -> (Status, Json<GetDirectChatHistoryOutput>) {
    let data = data.into_inner(); // Get the actual `MyData` struct

    let conn = Connection::open("my_database.db").unwrap();
    let msgs = message::get_chat_history(&conn, &data.username_a, &data.username_b);

    if msgs.is_ok() {
        let response = GetDirectChatHistoryOutput{
            messages: msgs.unwrap(),
        };
        (Status::Ok, Json(response))
    } else {
        let response = GetDirectChatHistoryOutput{
            messages: vec![],
        };
        (Status::BadRequest, Json(response))
    }
}
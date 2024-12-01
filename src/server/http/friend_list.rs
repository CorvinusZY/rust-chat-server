use rocket::get;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rusqlite::Connection;
use crate::db::{friendship, message};
use crate::db::message::Message;

//Get chat history for direct messaging
#[derive(Deserialize)]
struct GetFriendsInput {
    from_username: String,
}

#[derive(Serialize)]
struct GetFriendsOutput {
    to_usernames: Vec<String>,
}

#[get("/friends?<from_username>")]
pub async fn get_friends(from_username: String) -> (Status, Json<GetFriendsOutput>) {

    let conn = Connection::open("my_database.db").unwrap();
    let friendships = friendship::get_friends(&from_username ,&conn);

    if friendships.is_ok() {
        let usernames = friendships.unwrap().iter().map(|x| x.to_username.clone()).collect();
        let response = GetFriendsOutput{
            to_usernames: usernames,
        };
        (Status::Ok, Json(response))
    } else {
        let response = GetFriendsOutput{
            to_usernames: vec![],
        };
        (Status::BadRequest, Json(response))
    }
}
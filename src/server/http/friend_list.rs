use crate::db::user;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rusqlite::Connection;

//Get chat history for direct messaging
#[derive(Deserialize)]
struct GetFriendsInput {
    from_username: String,
}

#[derive(Serialize)]
struct GetFriendsOutput {
    to_usernames: Vec<String>,
    to_user_pictures: Vec<String>,
}

#[get("/friends?<from_username>")]
pub async fn get_friends(from_username: String) -> (Status, Json<GetFriendsOutput>) {
    let conn = Connection::open("my_database.db").unwrap();
    //let friendships = friendship::get_friends(&from_username, &conn);
    let friendships = user::get_friends_profile(&conn, from_username);

    if friendships.is_ok() {
        let friends = friendships.unwrap();
        let usernames: Vec<String> = friends.iter().map(|x| x.username.clone()).collect();
        let user_pictures: Vec<String> = friends.iter().map(|x| x.picture.clone()).collect();
        let response = GetFriendsOutput {
            to_usernames: usernames,
            to_user_pictures: user_pictures,
        };
        (Status::Ok, Json(response))
    } else {
        let response = GetFriendsOutput {
            to_usernames: vec![],
            to_user_pictures: vec![],
        };
        (Status::BadRequest, Json(response))
    }
}

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
    user_picture: String,
    to_usernames: Vec<String>,
    to_user_pictures: Vec<String>,
}

#[get("/friends?<from_username>")]
pub async fn get_friends(from_username: String) -> (Status, Json<GetFriendsOutput>) {
    let conn = Connection::open("my_database.db").unwrap();

    let friendships = user::get_friends_profile(&conn, from_username.clone());
    let user = user::get_by_username(&conn, from_username.clone());

    if friendships.is_ok() && user.is_ok() {
        let friends = friendships.unwrap();
        let usernames: Vec<String> = friends.iter().map(|x| x.username.clone()).collect();
        let friend_pictures: Vec<String> = friends.iter().map(|x| x.picture.clone()).collect();

        let user_picture = user.unwrap().picture;

        let response = GetFriendsOutput {
            user_picture: user_picture,
            to_usernames: usernames,
            to_user_pictures: friend_pictures,
        };
        (Status::Ok, Json(response))
    } else {
        let response = GetFriendsOutput {
            user_picture: String::new(),
            to_usernames: vec![],
            to_user_pictures: vec![],
        };
        (Status::BadRequest, Json(response))
    }
}

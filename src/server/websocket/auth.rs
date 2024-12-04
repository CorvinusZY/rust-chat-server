use crate::data::user::UserLoginCredential;
use crate::db::user;
use rocket::serde::Deserialize;
use rusqlite::Connection;
use serde::Serialize;
use serde_json::{self};
use warp::http::StatusCode;
use warp::ws::Ws;

static ALLOW_USERS: [&str; 2] = ["corvinus", "winnie"];

// Custom rejection for authentication failure
#[derive(Debug)]
struct AuthenticationError;
impl warp::reject::Reject for AuthenticationError {}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    auth: String, // The query parameter to extract
}

pub async fn authenticate(ws: Ws, params: QueryParams) -> Result<(Ws, String), warp::Rejection> {
    let auth_header = params.auth.clone();
    if !ALLOW_USERS.contains(&auth_header.as_str()) {
        return Err(warp::reject::custom(AuthenticationError));
    }
    println!("Successfully authenticated user {auth_header}");
    Ok((ws, auth_header))
}

pub async fn authenticate_password(
    credential: &UserLoginCredential,
) -> Result<String, warp::Rejection> {
    let username = credential.username.clone();
    let password = credential.password.clone();

    let conn = Connection::open("my_database.db").unwrap();
    let user = user::get_by_username(&conn, username);

    if user.unwrap().password != password {
        return Err(warp::reject::custom(AuthenticationError));
    }

    println!(
        "Successfully authenticated user {} and password",
        credential.username.clone()
    );
    Ok(credential.username.clone())
}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if !err.find::<AuthenticationError>().is_none() {
        // Return a 401 Unauthorized status with a JSON message
        let json = warp::reply::json(&ErrorResponse {
            message: "Unauthorized: Invalid credentials".to_string(),
        });
        Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED))
    } else {
        // Default rejection handling
        Err(err)
    }
}

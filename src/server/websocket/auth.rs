use crate::db::user;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rocket::serde::Deserialize;
use rusqlite::Connection;
use serde::Serialize;
use warp::http::StatusCode;
use warp::ws::Ws;

static ALLOW_USERS: [&str; 3] = ["corvinus", "winnie", "john"];

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
    name: String, // This is name. bad wording
    password: String,
}

pub async fn authenticate(ws: Ws, params: QueryParams) -> Result<(Ws, String), warp::Rejection> {
    let auth_header = params.name.clone();
    if !ALLOW_USERS.contains(&auth_header.as_str()) {
        return Err(warp::reject::custom(AuthenticationError));
    }

    let decode_password = decode_password(params.password.clone()).unwrap();
    if authenticate_password(params.name.clone(), decode_password) {
        Ok((ws, auth_header))
    } else {
        Err(warp::reject::custom(AuthenticationError))
    }
}

pub fn decode_password(encoded_password: String) -> Result<String, base64::DecodeError> {
    let decoded_bytes = STANDARD.decode(encoded_password)?;
    let decoded_password = String::from_utf8(decoded_bytes).expect("Invalid UTF-8 sequence");
    Ok(decoded_password)
}

pub fn authenticate_password(username: String, password: String) -> bool {
    let conn = Connection::open("my_database.db").unwrap();
    let user = user::get_by_username(&conn, username.clone());

    if user.unwrap().password != password {
        return false;
    }
    println!(
        "Successfully authenticated user {} and password",
        username.clone()
    );
    true
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

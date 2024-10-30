use serde::Serialize;
use warp::http::StatusCode;
use warp::ws::Ws;

static ALLOW_USERS: [&str; 2] = ["A", "B"];

// Custom rejection for authentication failure
#[derive(Debug)]
struct AuthenticationError;
impl warp::reject::Reject for AuthenticationError {}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn authenticate(ws: Ws, auth_header: String) -> Result<(Ws, String), warp::Rejection> {
    if !ALLOW_USERS.contains(&auth_header.as_str()) {
        return Err(warp::reject::custom(AuthenticationError));
    }
    println!("Successfully authenticated user {auth_header}");
    Ok((ws, auth_header))
}

// Handle rejections (e.g., failed authentication)

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

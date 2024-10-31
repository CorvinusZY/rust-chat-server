use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{get, post};

#[derive(Deserialize)]
struct MyData {
    message: String,
}

#[post("/echo", data = "<data>")]
pub async fn json_handler(data: Json<MyData>) -> String {
    let data = data.into_inner(); // Get the actual `MyData` struct
    format!("Received message: {}", data.message)
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, this is the Rocket HTTP server!\n"
}

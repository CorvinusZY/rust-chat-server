use rocket::{get, post, routes};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;

#[derive(Deserialize)]
struct MyData {
    message: String,
}

#[post("/echo", data = "<data>")]
async fn json_handler(data: Json<MyData>) -> String {
    let data = data.into_inner(); // Get the actual `MyData` struct
    format!("Received message: {}", data.message)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, this is the Rocket HTTP server!\n"
}

pub async fn init() {
    let rocket = rocket::build().mount("/", routes![index, json_handler]).launch().await;
}
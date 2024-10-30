use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, this is the Rocket HTTP server!\n"
}

pub async fn init() {
    let rocket = rocket::build().mount("/", routes![index]).launch().await;
}
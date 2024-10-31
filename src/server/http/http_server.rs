use crate::server::http::test_endpoints;

use rocket::{routes};
use test_endpoints::index;
use test_endpoints::json_handler;

pub async fn init() {
    let rocket = rocket::build()
        .mount("/", routes![index, json_handler])
        .launch()
        .await;
}

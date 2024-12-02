use crate::server::http::friend_list::get_friends;
use crate::server::http::message::get_chat_history;
use crate::server::http::test_endpoints;
use rocket::routes;
use test_endpoints::index;
use test_endpoints::json_handler;

pub async fn init() {
    let rocket = rocket::build()
        .mount(
            "/",
            routes![index, json_handler, get_chat_history, get_friends],
        )
        .launch()
        .await;
}

use rocket::form::validate::range;
use crate::db::{friendship, message, user};
use crate::db::user::User;
use rusqlite::Connection;

pub fn prepare_db(conn: &Connection) {
    println!("Preparing DB...");
    // prepare users
    let users = [
        User {
            id: 1,
            username: "corvinus".to_string(),
            password: "123".to_string(),
        },
        User {
            id: 2,
            username: "winnie".to_string(),
            password: "456".to_string(),
        },
        User {
            id: 3,
            username: "john".to_string(),
            password: "789".to_string(),
        }
    ];

    user::create_table(&conn, &users);
    user::inspect_users(&conn);

    // prepare msgs
    message::create_table(&conn);

    // prepare friendships
    friendship::create_table(&conn);
    for (i, from_user) in users.iter().enumerate() {
        if i == users.len() - 1 {break}
        for to_user in &users[i+1..] {
            friendship::insert(&from_user.username, &to_user.username, &conn);
        }
    }
    friendship::inspect_friendships(&conn);
}

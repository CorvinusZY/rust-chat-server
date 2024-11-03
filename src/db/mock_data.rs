use crate::db::{message, user};
use crate::db::user::User;
use rusqlite::Connection;

pub fn prepare_db(conn: &Connection) {
    println!("Preparing DB...");
    // prepare users
    let users = [User {
        id: 1,
        username: "corvinus".to_string(),
        password: "123".to_string(),
    }];
    user::create_table(&conn, &users);
    user::inspect_users(&conn);

    // prepare msgs
    message::create_table(&conn);
}

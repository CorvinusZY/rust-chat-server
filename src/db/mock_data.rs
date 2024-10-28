use once_cell::sync::Lazy;
use rusqlite::Connection;
use uuid::Uuid;
use crate::db::user;
use crate::db::user::User;


pub fn prepare_db(conn: &Connection) {
    println!("Preparing DB...");
    // prepare users
    let users = [
        User {
            id: 1,
            username: "corvinus".to_string(),
            password: "123".to_string(),
        }
    ];
    user::create_table(&conn, &users);
    user::inspect_users(&conn);
}

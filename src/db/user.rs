use rusqlite::{params, Connection};

// Define a struct to represent a User
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub picture: String,
}

// Table Utils
pub fn create_table(conn: &Connection, users: &[User]) {
    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            picture TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    println!("User table checked or created"); // not checking if table exists for cleanness
    create_prepared_users(conn, users);
    println!("User records created");
}

fn create_prepared_users(conn: &Connection, users: &[User]) {
    for user in users {
        // Insert record into the table
        let inserted_rows = conn
            .execute(
                "INSERT OR IGNORE INTO user (id, username, password, picture) VALUES (?1, ?2, ?3, ?4)",
                params![&user.id, &user.username, &user.password, &user.picture],
            )
            .unwrap();

        let username = &user.username;
        if inserted_rows > 0 {
            println!("User created: {username}");
        } else {
            println!("User already exists: {username}");
        }
    }
}

pub fn inspect_users(conn: &Connection) {
    println!("Inspecting users");
    // Query and print data
    let mut stmt = conn
        .prepare("SELECT id, username, password, picture FROM user")
        .unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                picture: row.get(3)?,
            })
        })
        .unwrap();

    println!("Persons in the database:");
    for person in person_iter {
        println!("{:?}", person.unwrap());
    }
}
pub fn get_by_username(conn: &Connection, username: String) -> Result<User, rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT id, username, password, picture FROM user WHERE username = ?1")?;
    let user_row = stmt.query_row(params![username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            picture: row.get(3)?,
        })
    })?;
    Ok(user_row)
}

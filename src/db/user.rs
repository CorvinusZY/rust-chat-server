use rusqlite::{params, Connection};

// Define a struct to represent a User
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
}

pub fn create_table(conn: &Connection) {
    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    ).unwrap();
    println!("User table created");
    create_prepared_users(conn);
    println!("User records created");
}

fn create_prepared_users(conn: &Connection) {
    // Insert a new person into the table
    let username = "corvinus";
    let password = "123";
    let inserted_rows = conn.execute(
        "INSERT OR IGNORE INTO user (username, password) VALUES (?1, ?2)",
        params![username, password],
    ).unwrap();
    if inserted_rows > 0 {
        println!("User created: {username}");
    } else {
        println!("User already exists: {username}");
    }
}

pub fn inspect_users(conn: &Connection) {
    println!("Inspecting users");
    // Query and print data
    let mut stmt = conn.prepare("SELECT id, username, password FROM user").unwrap();
    let person_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    }).unwrap();

    println!("Persons in the database:");
    for person in person_iter {
        println!("{:?}", person.unwrap());
    }
}

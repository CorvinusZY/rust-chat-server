use rocket::serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};

// tracks direct friendship in both directions
#[derive(Debug, Serialize, Deserialize)]
pub struct Friendship {
    pub id: i64,
    pub from_username: String,
    pub to_username: String,
}

// Query utils
pub fn get_friends(
    from_username: &str,
    conn: &Connection,
) -> Result<Vec<Friendship>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, from_username, to_username
         FROM friendships
         WHERE from_username = ?1",
    )?;

    let friends = stmt
        .query_map(params![from_username], |row| {
            Ok(Friendship {
                id: row.get(0)?,
                from_username: row.get(1)?,
                to_username: row.get(2)?,
            })
        })
        .unwrap();
    let friendships: Result<Vec<Friendship>, rusqlite::Error> = friends.collect(); // Collect the results into a Vec<Message>
    friendships
}

pub fn insert(username_a: &str, username_b: &str, conn: &Connection) {
    conn.execute(
        "INSERT OR IGNORE INTO friendships (from_username, to_username) VALUES (?, ?)",
        params![username_a, username_b,],
    )
    .unwrap();
    let id = conn.last_insert_rowid();
    println!("Forward friendship created: {id}");

    conn.execute(
        "INSERT OR IGNORE INTO friendships (from_username, to_username) VALUES (?, ?)",
        params![username_b, username_a,],
    )
    .unwrap();
    let id = conn.last_insert_rowid();
    println!("Backward friendship created: {id}");
}

// Table Utils
pub fn create_table(conn: &Connection) {
    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS friendships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            from_username TEXT NOT NULL,
            to_username TEXT NOT NULL,
            UNIQUE(from_username, to_username)
        )",
        [],
    )
    .unwrap();
    println!("Friendship table checked or created"); // not checking if table exists for cleanness
}

// Test Utils
pub fn inspect_friendships(conn: &Connection) {
    println!("Inspecting friendships");
    // Query and print data
    let mut stmt = conn
        .prepare("SELECT id, from_username, to_username FROM friendships")
        .unwrap();
    let friendships_iter = stmt
        .query_map([], |row| {
            Ok(Friendship {
                id: row.get(0)?,
                from_username: row.get(1)?,
                to_username: row.get(2)?,
            })
        })
        .unwrap();

    println!("Friendships in the database:");
    for friendship in friendships_iter {
        println!("{:?}", friendship.unwrap());
    }
}

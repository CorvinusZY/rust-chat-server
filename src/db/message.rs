use crate::data::message::IncomingMessage;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum MessageType {
    Direct,
    Group,
}
impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::Direct => "direct".to_string(),
            MessageType::Group => "group".to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub sender: String,
    pub receiver: String, // can be either a group or a user
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub message_type: MessageType,
}

// Record Utils
impl Message {
    pub fn delete(&self, conn: &Connection) {
        conn.execute(
            "DELETE FROM messages WHERE message_id = ?1",
            params![self.message_id],
        )
        .unwrap();

        let id = self.message_id;
        println!("Msg delete: {id}");
    }
}

pub fn get_chat_history(
    conn: &Connection,
    username_1: &str,
    username_2: &str,
) -> Result<Vec<Message>, rusqlite::Error> {
    let mut stmt = conn
        .prepare(
            "
            SELECT message_id, sender, receiver, created_at, message, message_type
            FROM messages
            WHERE (sender = ?1 AND receiver = ?2)
               OR (sender = ?2 AND receiver = ?1)
            ORDER BY created_at ASC",
        )
        .unwrap();

    let message_rows = stmt
        .query_map(params![username_1, username_2], |row| {
            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap()
                .with_timezone(&Utc);
            Ok(Message {
                message_id: row.get(0)?,
                sender: row.get(1)?,
                receiver: row.get(2)?,
                created_at,
                message: row.get(4)?,
                message_type: match row.get::<_, String>(5)?.as_str() {
                    "direct" => MessageType::Direct,
                    "group" => MessageType::Group,
                    _ => unreachable!(),
                },
            })
        })
        .unwrap();

    let messages: Result<Vec<Message>, rusqlite::Error> = message_rows.collect(); // Collect the results into a Vec<Message>
    messages
}

pub fn get_by_id(conn: &Connection, message_id: i32) -> Result<Message, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT message_id, sender, receiver, created_at, message, message_type FROM messages WHERE message_id = ?1")?;
    let message_row = stmt.query_row(params![message_id], |row| {
        Ok(Message {
            message_id: row.get(0)?,
            sender: row.get(1)?,
            receiver: row.get(2)?,
            created_at: DateTime::parse_from_rfc3339(row.get::<_, String>(3)?.as_str())
                .unwrap()
                .with_timezone(&Utc), // Parse to Utc
            message: row.get(4)?,
            message_type: match row.get::<_, String>(5)?.as_str() {
                "direct" => MessageType::Direct,
                "group" => MessageType::Group,
                _ => unreachable!(),
            },
        })
    })?;
    Ok(message_row)
}

pub fn insert(msg: &IncomingMessage, conn: &Connection) {
    let now = Utc::now().to_rfc3339(); // Get the current time
    conn.execute(
        "INSERT INTO messages (sender, receiver, created_at, message, message_type)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            msg.sender,
            msg.receiver,
            now, // Convert to string for SQLite
            msg.content,
            msg.message_type,
        ],
    )
    .unwrap();

    let id = conn.last_insert_rowid();
    println!("Msg created / updated: {id}");
}

pub fn insert_direct_record(msg: &Message, conn: &Connection) {
    conn.execute(
        "INSERT OR IGNORE INTO messages (message_id, sender, receiver, created_at, message, message_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
                msg.message_id,
                msg.sender,
                msg.receiver,
                msg.created_at.to_rfc3339(), // Convert to string for SQLite
                msg.message,
                msg.message_type.to_string(),
            ],
    ).unwrap();

    let id = conn.last_insert_rowid();
    println!("Msg created / updated: {id}");
}

// Table Utils
pub fn create_table(conn: &Connection) {
    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            message_id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender TEXT NOT NULL,
            receiver TEXT NOT NULL,
            created_at TEXT NOT NULL,
            message TEXT NOT NULL,
            message_type TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    println!("Message table checked or created"); // not checking if table exists for cleanness
}

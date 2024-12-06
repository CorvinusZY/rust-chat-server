use crate::db::message::{Message, MessageType};
use crate::db::user::User;
use crate::db::{friendship, message, user};
use chrono::{NaiveDateTime, TimeZone, Utc};
use rusqlite::Connection;

pub fn prepare_db(conn: &Connection) {
    println!("Preparing DB...");
    // prepare users
    let users = [
        User {
            id: 1,
            username: "corvinus".to_string(),
            password: "123".to_string(),
            picture: "https://chatscope.io/storybook/react/assets/eliot-JNkqSAth.svg".to_string(),
        },
        User {
            id: 2,
            username: "winnie".to_string(),
            password: "456".to_string(),
            picture: "https://chatscope.io/storybook/react/assets/lilly-aj6lnGPk.svg".to_string(),
        },
        User {
            id: 3,
            username: "john".to_string(),
            password: "789".to_string(),
            picture: "https://chatscope.io/storybook/react/assets/zoe-E7ZdmXF0.svg".to_string(),
        },
    ];

    user::create_table(&conn, &users);
    user::inspect_users(&conn);

    // prepare msgs
    // Convert it to DateTime<Utc> if you want to use time zone info
    let datetime_msg1 = Utc.from_utc_datetime(
        &NaiveDateTime::parse_from_str("2023-12-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    let datetime_msg2 = Utc.from_utc_datetime(
        &NaiveDateTime::parse_from_str("2023-12-01 12:42:01", "%Y-%m-%d %H:%M:%S").unwrap(),
    );

    let msgs = [
        Message {
            message_id: 1,
            sender: "winnie".to_string(),
            receiver: "corvinus".to_string(),
            created_at: datetime_msg1,
            message: "hello teammate".to_string(),
            message_type: MessageType::Direct,
        },
        Message {
            message_id: 2,
            sender: "corvinus".to_string(),
            receiver: "winnie".to_string(),
            created_at: datetime_msg2,
            message: "hello hello".to_string(),
            message_type: MessageType::Direct,
        },
    ];
    message::create_table(&conn);
    for msg in msgs {
        message::insert_direct_record(&msg, &conn);
    }

    // prepare friendships
    friendship::create_table(&conn);
    for (i, from_user) in users.iter().enumerate() {
        if i == users.len() - 1 {
            break;
        }
        for to_user in &users[i + 1..] {
            friendship::insert(&from_user.username, &to_user.username, &conn);
        }
    }
    friendship::inspect_friendships(&conn);
}

use crate::server::auth;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::os::unix::raw::time_t;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket, Ws};
use warp::Filter;

type Users = Arc<Mutex<HashMap<String, SplitSink<WebSocket, Message>>>>;
// Define the global users map
static USERS: Lazy<Arc<Mutex<HashMap<String, SplitSink<WebSocket, Message>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Define the structure for JSON messages
#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    sent_at: String,
    from_id: String,
    to_id: String,
    message_type: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseMessage {
    response_type: String,
    content: String,
}

pub async fn start_server() {
    // Define the WebSocket route
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::header::<String>("authorization")) // Extract the 'Authorization' header
        .and_then(auth::authenticate)
        .map(|(ws, username): (Ws, String)| {
            let username_clone = username.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, username_clone))
        })
        .recover(auth::handle_rejection);

    // Start the WebSocket server
    println!("WebSocket server running at ws://127.0.0.1:3030/ws");
    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_connection(websocket: WebSocket, username: String) {
    println!("New websocket connection established");

    let (mut ws_tx, mut ws_rx) = websocket.split();
    // Add the user to the shared map
    {
        let mut users_lock = USERS.lock().await;
        users_lock.insert(username.clone(), ws_tx); // Save the sender in the map
    }

    // Receiving messages from the client
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Ok(text) = message.to_str() {
            // Parse the incoming JSON message
            if let Ok(incoming) = serde_json::from_str::<IncomingMessage>(text) {
                println!("Received from client: {:?}", incoming);

                // Create a response message
                let response = ResponseMessage {
                    response_type: "echo".to_string(),
                    content: format!("Echo from {}: {}", &username, incoming.content),
                };

                // Serialize response to JSON and send it
                let response_text = serde_json::to_string(&incoming).unwrap();
                // add delay for 2 seconds
                thread::sleep(Duration::from_secs(2));

                let mut users_lock = USERS.lock().await;
                if let sendOption = users_lock.get_mut(&incoming.to_id) {
                    if sendOption.is_none() {
                        println!("Failed to send response to user '{}': user not online", username);
                    } else {
                        let sender = sendOption.unwrap();
                        let _ = sender.send(Message::text(response_text)).await;
                    }
                }
                println!("Response sended");
            }
        }
    }

    // Remove the user from the global map when disconnected
    {
        let mut users_lock = USERS.lock().await;
        users_lock.remove(&username);
    }

    println!("{} disconnected", username);
}

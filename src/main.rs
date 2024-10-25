use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use std::thread;
use std::time::Duration;
use warp::Filter;
use serde::{Deserialize, Serialize};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use futures::stream::SplitSink;
use warp::ws::{WebSocket, Ws};
use warp::ws::Message;
use serde_json::json;
use warp::http::StatusCode;

static ALLOW_USERS: [&str; 2] = ["A", "B"];
type Users = Arc<Mutex<HashMap<String,  SplitSink<WebSocket, Message>>>>;
// Define the global users map
static USERS: Lazy<Arc<Mutex<HashMap<String, SplitSink<WebSocket, Message>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Define the structure for JSON messages
#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
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

// Custom rejection for authentication failure
#[derive(Debug)]
struct AuthenticationError;

impl warp::reject::Reject for AuthenticationError {}


#[tokio::main]
async fn main() {
    // Define the WebSocket route
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::header::<String>("authorization")) // Extract the 'Authorization' header
        .and_then(authenticate)
        .map(|(ws, username): (Ws, String)| {
            let username_clone = username.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, username_clone))
        })
        .recover(handle_rejection);

    // Start the WebSocket server
    println!("WebSocket server running at ws://127.0.0.1:3030/ws");
    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn authenticate(ws: Ws, auth_header: String) -> Result<(Ws,String), warp::Rejection> {
    if !ALLOW_USERS.contains(&auth_header.as_str()) {
        return Err(warp::reject::custom(AuthenticationError))
    }
    println!("Successfully authenticated user {auth_header}");
    Ok((ws,auth_header))
}


// Handle rejections (e.g., failed authentication)
// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if  !err.find::<AuthenticationError>().is_none() {
        // Return a 401 Unauthorized status with a JSON message
        let json = warp::reply::json(&ErrorResponse {
            message: "Unauthorized: Invalid credentials".to_string(),
        });
        Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED))
    } else {
        // Default rejection handling
        Err(err)
    }
}

async fn handle_connection(websocket: WebSocket, username: String) {
    println!("New websocket connection established");

    let (mut ws_tx, mut ws_rx) = websocket.split();
    let user_name = username.to_string();
    // Add the user to the shared map
    {
        let mut users_lock = USERS.lock().await;
        users_lock.insert(user_name.clone(), ws_tx);  // Save the sender in the map
    }

    //let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<String>();
    // Receiving messages from the client
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Ok(text) = message.to_str() {
            // Parse the incoming JSON message
            if let Ok(incoming) = serde_json::from_str::<IncomingMessage>(text) {
                println!("Received from client: {:?}", incoming);

                // Create a response message
                let response = ResponseMessage {
                    response_type: "echo".to_string(),
                    content: format!("Echo from {}: {}", &user_name, incoming.content),
                };

                // Serialize response to JSON and send it
                let response_text = serde_json::to_string(&response).unwrap();
                // outgoing_tx.send(response_text).unwrap();
                // add delay for 2 seconds
                thread::sleep(Duration::from_secs(2));

                let mut users_lock = USERS.lock().await;
                if let Some(sender) = users_lock.get_mut(&user_name) {
                    let _ = sender.send(Message::text(response_text)).await;
                }
                println!("Response sended");
                // ws_tx.send(Message::text(response_text))
                //     .await
                //     .unwrap();
            }
        }
    };

    // Remove the user from the global map when disconnected
    {
        let mut users_lock = USERS.lock().await;
        users_lock.remove(&user_name);
    }

    println!("{} disconnected", user_name);
    // Sending messages to the client
    // while let Some(outgoing_message) = outgoing_rx.recv().await {
    //     // add delay for 2 seconds
    //     thread::sleep(Duration::from_secs(2));
    //     ws_tx.send(Message::text(outgoing_message))
    //         .await
    //         .unwrap();
    // }
}



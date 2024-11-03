use rocket::serde::{Deserialize, Serialize};

// Define the structure for JSON messages
#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub(crate) sender_id: String,
    pub(crate) receiver_id: String,
    pub(crate) message_type: String,
    pub(crate) content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub(crate) response_type: String,
    pub(crate) content: String,
}
use rocket::serde::{Deserialize, Serialize};

// Define the structure for JSON user login credentials
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginCredential {
    pub(crate) username: String,
    pub(crate) password: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginResponse {
    pub jwt: String,
}
